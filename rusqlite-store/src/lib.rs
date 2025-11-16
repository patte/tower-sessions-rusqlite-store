use async_trait::async_trait;
use rusqlite::OptionalExtension;
use std::error::Error;
use time::OffsetDateTime;
pub use tokio_rusqlite;
use tokio_rusqlite::{params, Connection, Result as SqlResult};
use tower_sessions_core::{
    session::{Id, Record},
    session_store, ExpiredDeletion, SessionStore,
};

/// An error type for Rusqlite stores.
#[derive(thiserror::Error, Debug)]
pub enum RusqliteStoreError {
    /// A variant to map `tokio_rusqlite` errors.
    #[error(transparent)]
    TokioRusqlite(#[from] tokio_rusqlite::Error),

    /// A variant to map `rmp_serde` encode errors.
    #[error(transparent)]
    Encode(#[from] rmp_serde::encode::Error),

    /// A variant to map `rmp_serde` decode errors.
    #[error(transparent)]
    Decode(#[from] rmp_serde::decode::Error),

    /// A variant for other backend errors.
    #[error("Backend error: {0}")]
    Other(String),
}

impl From<RusqliteStoreError> for session_store::Error {
    fn from(err: RusqliteStoreError) -> Self {
        match err {
            RusqliteStoreError::TokioRusqlite(inner) => {
                session_store::Error::Backend(inner.to_string())
            }
            RusqliteStoreError::Decode(inner) => session_store::Error::Decode(inner.to_string()),
            RusqliteStoreError::Encode(inner) => session_store::Error::Encode(inner.to_string()),
            RusqliteStoreError::Other(inner) => session_store::Error::Backend(inner),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RusqliteStore {
    conn: Connection,
    table_name: String,
}

impl RusqliteStore {
    /// Create a new SQLite store with the provided connection.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use tower_sessions_rusqlite_store::{tokio_rusqlite::Connection, RusqliteStore};
    ///
    /// # tokio_test::block_on(async {
    /// let conn = Connection::open_in_memory().await.unwrap();
    /// let session_store = RusqliteStore::new(conn);
    /// # })
    /// ```
    pub fn new(conn: Connection) -> Self {
        Self {
            conn,
            table_name: "tower_sessions".into(),
        }
    }

    /// Set the session table name with the provided name.
    pub fn with_table_name(mut self, table_name: impl AsRef<str>) -> Result<Self, String> {
        let table_name = table_name.as_ref();
        if !is_valid_table_name(table_name) {
            return Err(format!(
                "Invalid table name '{}'. Table names must be alphanumeric and may contain \
                 hyphens or underscores.",
                table_name
            ));
        }

        self.table_name = table_name.to_owned();
        Ok(self)
    }

    /// Migrate the session schema.
    pub async fn migrate(&self) -> SqlResult<()> {
        let conn = self.conn.clone();
        let query = format!(
            r#"
            create table if not exists {}
            (
                id text primary key not null,
                data blob not null,
                expiry_date integer not null
            )
            "#,
            self.table_name
        );
        conn.call(move |conn| conn.execute(&query, [])).await?;

        Ok(())
    }
}

fn id_exists_with_conn(
    conn: &rusqlite::Connection,
    table_name: &str,
    id: &Id,
) -> rusqlite::Result<bool> {
    let query = format!(
        r#"
        select exists(select 1 from {} where id = ?1)
        "#,
        table_name
    );
    let mut stmt = conn.prepare(&query)?;
    stmt.query_row(params![id.to_string()], |row| row.get(0))
}

fn save_with_conn(
    conn: &rusqlite::Connection,
    table_name: &str,
    record: &Record,
    record_data: &[u8],
) -> rusqlite::Result<usize> {
    let query = format!(
        r#"
        insert into {}
            (id, data, expiry_date) values (?1, ?2, ?3)
        on conflict(id) do update set
            data = excluded.data,
            expiry_date = excluded.expiry_date
        "#,
        table_name
    );
    conn.execute(
        &query,
        params![
            record.id.to_string(),
            record_data,
            record.expiry_date.unix_timestamp()
        ],
    )
}

#[async_trait]
impl ExpiredDeletion for RusqliteStore {
    async fn delete_expired(&self) -> session_store::Result<()> {
        let conn = self.conn.clone();
        let query = format!(
            r#"
            delete from {table_name}
            where expiry_date < ?1
            "#,
            table_name = self.table_name
        );
        conn.call(move |conn| conn.execute(&query, [OffsetDateTime::now_utc().unix_timestamp()]))
            .await
            .map_err(|e| {
                // printing the error here because this usually runs in the background
                // and thus the error is only received shortly before the process exits
                eprintln!("Error deleting expired sessions: {:?}", e);
                RusqliteStoreError::TokioRusqlite(e)
            })?;

        Ok(())
    }
}

#[async_trait]
impl SessionStore for RusqliteStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        let conn = self.conn.clone();

        let new_id = conn
            .call({
                let mut record = record.clone();
                let table_name = self.table_name.clone();

                move |conn| {
                    let tx = conn.transaction()?;

                    while id_exists_with_conn(&tx, &table_name, &record.id)? {
                        record.id = Id::default();
                    }

                    let record_data = rmp_serde::to_vec(&record).map_err(Box::new)?;

                    save_with_conn(&tx, &table_name, &record, &record_data)?;

                    tx.commit()?;

                    Ok(record.id)
                }
            })
            .await
            .map_err(
                |e: tokio_rusqlite::Error<Box<dyn Error + Send + Sync>>| match e {
                    tokio_rusqlite::Error::Error(boxed_err) => {
                        match boxed_err.downcast::<rmp_serde::encode::Error>() {
                            Ok(encode_error) => RusqliteStoreError::Encode(*encode_error),
                            Err(original_box) => {
                                RusqliteStoreError::Other(original_box.to_string())
                            }
                        }
                    }
                    other => RusqliteStoreError::Other(other.to_string()),
                },
            )?;

        record.id = new_id;

        Ok(())
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let conn = self.conn.clone();
        let table_name = self.table_name.clone();
        let record = record.clone();
        let record_data = rmp_serde::to_vec(&record).map_err(RusqliteStoreError::Encode)?;

        conn.call(move |conn| save_with_conn(conn, &table_name, &record, &record_data))
            .await
            .map_err(RusqliteStoreError::TokioRusqlite)?;

        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let conn = self.conn.clone();

        let data = conn
            .call({
                let table_name = self.table_name.clone();
                let session_id = session_id.to_string();
                move |conn| {
                    let query = format!(
                        r#"
                        select data from {}
                        where id = ?1 and expiry_date > ?2
                        "#,
                        table_name
                    );
                    let mut stmt = conn.prepare(&query)?;
                    stmt.query_row(
                        params![session_id, OffsetDateTime::now_utc().unix_timestamp()],
                        |row| {
                            let data: Vec<u8> = row.get(0)?;
                            Ok(data)
                        },
                    )
                    .optional()
                }
            })
            .await
            .map_err(RusqliteStoreError::TokioRusqlite)?;

        match data {
            Some(data) => {
                let record: Record =
                    rmp_serde::from_slice(&data).map_err(RusqliteStoreError::Decode)?;
                Ok(Some(record))
            }
            None => Ok(None),
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let conn = self.conn.clone();

        conn.call({
            let table_name = self.table_name.clone();
            let session_id = session_id.to_string();
            move |conn| {
                let query = format!(
                    r#"
                    delete from {} where id = ?1
                    "#,
                    table_name
                );
                conn.execute(&query, params![session_id])
            }
        })
        .await
        .map_err(RusqliteStoreError::TokioRusqlite)?;

        Ok(())
    }
}

fn is_valid_table_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

// unit tests from https://github.com/maxcountryman/tower-sessions/blob/6ad8933b4f5e71f3202f0c1a28f194f3db5234c8/memory-store/src/lib.rs#L62
#[cfg(test)]
mod rusqlite_store_tests {
    use time::Duration;

    use super::*;

    async fn create_store() -> RusqliteStore {
        let conn = Connection::open_in_memory().await.unwrap();
        let store = RusqliteStore::new(conn);
        store.migrate().await.unwrap();
        store
    }

    #[tokio::test]
    async fn test_create() {
        let store = create_store().await;
        let mut record = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date: OffsetDateTime::now_utc() + Duration::minutes(30),
        };
        assert!(store.create(&mut record).await.is_ok());
    }

    #[tokio::test]
    async fn test_save() {
        let store = create_store().await;
        let record = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date: OffsetDateTime::now_utc() + Duration::minutes(30),
        };
        assert!(store.save(&record).await.is_ok());
    }

    #[tokio::test]
    async fn test_load() {
        let store = create_store().await;
        let mut record = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date: OffsetDateTime::now_utc() + Duration::minutes(30),
        };
        store.create(&mut record).await.unwrap();
        let loaded_record = store.load(&record.id).await.unwrap();
        assert_eq!(Some(record), loaded_record);
    }

    #[tokio::test]
    async fn test_delete() {
        let store = create_store().await;
        let mut record = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date: OffsetDateTime::now_utc() + Duration::minutes(30),
        };
        store.create(&mut record).await.unwrap();
        assert!(store.delete(&record.id).await.is_ok());
        assert_eq!(None, store.load(&record.id).await.unwrap());
    }

    #[tokio::test]
    async fn test_create_id_collision() {
        let store = create_store().await;
        let expiry_date = OffsetDateTime::now_utc() + Duration::minutes(30);
        let mut record1 = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date,
        };
        let mut record2 = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date,
        };
        store.create(&mut record1).await.unwrap();
        record2.id = record1.id; // Set the same ID for record2
        store.create(&mut record2).await.unwrap();
        assert_ne!(record1.id, record2.id); // IDs should be different
    }

    #[tokio::test]
    async fn test_delete_expired() {
        let store = create_store().await;
        let mut record = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date: OffsetDateTime::now_utc() - Duration::minutes(30),
        };
        store.create(&mut record).await.unwrap();
        store.delete_expired().await.unwrap();
        assert_eq!(None, store.load(&record.id).await.unwrap());
    }
}
