<h1 align="center">
    tower-sessions-rusqlite-store
</h1>

<p align="center">
    (tokio-)rusqlite session store for <code>tower-sessions</code>.
</p>

[![tests](https://github.com/patte/tower-sessions-rusqlite-store/actions/workflows/rust.yml/badge.svg)](https://github.com/patte/tower-sessions-rusqlite-store/actions/workflows/rust.yml) [![crates.io](https://img.shields.io/crates/v/tower-sessions-rusqlite-store)](https://crates.io/crates/tower-sessions-rusqlite-store)


## Overview
This is a `SessionStore` for the [`tower-sessions`](https://github.com/maxcountryman/tower-sessions) middleware which uses [tokio-rusqlite](https://github.com/programatik29/tokio-rusqlite) for handling SQLite databases.

It is directly based on the [`sqlx-store`](https://github.com/maxcountryman/tower-sessions-stores/tree/main/sqlx-store) and uses the same folder structure as [tower-session-stores](https://github.com/maxcountryman/tower-sessions-stores) for easy maintenance.

All contributions are welcome!

## 🤸 Usage
Check out the [counter example](./rusqlite-store/examples/counter.rs). Run it with `cargo run --example counter`.

## 🧪 Tests
The tests are copied from [tower-session-stores](https://github.com/maxcountryman/tower-sessions-stores). Run them with `cargo nextest run rusqlite_store_test --test test_integration`.

## 🦺 Disclaimer
This is an unofficial fork of the original `tower-sessions-stores`. I'm relatively new to Rust and might have made stupid mistakes.

## 🙏 Credits
All credits go to the original authors of `tower-sessions-stores` and `tower-sessions`.

<!-- 📦 Release
cargo publish --dry-run -p tower-sessions-rusqlite-store
-->