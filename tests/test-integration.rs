#[macro_use]
mod common;

#[cfg(test)]
mod rusqlite_store_tests {
    use axum::Router;
    use tower_sessions::SessionManagerLayer;
    use tower_sessions_rusqlite_store::{tokio_rusqlite::Connection, RusqliteStore};

    use crate::common::build_app;

    async fn app(max_age: Option<Duration>) -> Router {
        let conn = Connection::open_in_memory().await.unwrap();
        let session_store = RusqliteStore::new(conn);
        session_store.migrate().await.unwrap();
        let session_manager = SessionManagerLayer::new(session_store).with_secure(true);

        build_app(session_manager, max_age)
    }

    route_tests!(app);
}
