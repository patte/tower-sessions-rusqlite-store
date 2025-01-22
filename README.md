<h1 align="center">
    tower-sessions-rusqlite-store
</h1>

<p align="center">
    (tokio-)rusqlite session store for <code>tower-sessions</code>.
</p>

[![tests](https://github.com/patte/tower-sessions-rusqlite-store/actions/workflows/rust.yml/badge.svg)](https://github.com/patte/tower-sessions-rusqlite-store/actions/workflows/rust.yml) [![crates.io](https://img.shields.io/crates/v/tower-sessions-rusqlite-store)](https://crates.io/crates/tower-sessions-rusqlite-store) [![codecov](https://codecov.io/gh/patte/tower-sessions-rusqlite-store/graph/badge.svg?token=FZIFCIHNMB)](https://codecov.io/gh/patte/tower-sessions-rusqlite-store)


## Overview
This is a `SessionStore` for the [`tower-sessions`](https://github.com/maxcountryman/tower-sessions) middleware which uses [tokio-rusqlite](https://github.com/programatik29/tokio-rusqlite) for handling SQLite databases.

It is directly based on the [`sqlx-store`](https://github.com/maxcountryman/tower-sessions-stores/tree/main/sqlx-store) and uses the same folder structure as [tower-session-stores](https://github.com/maxcountryman/tower-sessions-stores) for easy maintenance.

All contributions are welcome!

## 🤸 Usage
Check out the [counter example](./rusqlite-store/examples/counter.rs). Run it with `cargo run --example counter`.

## 🧪 Tests
This crate is covered by integration- and unit-tests.
The integration tests are copied from [tower-session-stores](https://github.com/maxcountryman/tower-sessions-stores) and kept in the `tests` create. They can be run with `cargo nextest run rusqlite_store_tests --test test_integration`.

The unit tests are copied from [maxcountryman/tower-sessions/memory-store](https://github.com/maxcountryman/tower-sessions/blob/6ad8933b4f5e71f3202f0c1a28f194f3db5234c8/memory-store/src/lib.rs#L62) and located directly in `src/lib.rs`. They can be run with `cargo nextest run rusqlite_store_tests -p tower-sessions-rusqlite-store`.

Run all tests with: `cargo nextest run rusqlite_store_tests`.

## 🦺 Disclaimer
This is an unofficial fork of the original `tower-sessions-stores`.

## 🙏 Credits
Most credits go to the original authors of `tower-sessions-stores` and `tower-sessions`.

<!-- 📦 Release
cargo publish --dry-run -p tower-sessions-rusqlite-store
-->