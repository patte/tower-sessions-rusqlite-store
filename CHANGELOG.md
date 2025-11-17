# 0.14.1

- bump rusqlite (0.32.1 -> 0.37.0), tokio-rusqlite (0.6.0 -> 0.7.0) and other dependencies by @Chleba in https://github.com/patte/tower-sessions-rusqlite-store/pull/29
- add test for session expiration by @patte in https://github.com/patte/tower-sessions-rusqlite-store/pull/26
- bump actions/checkout from 4 to 5 by @dependabot in https://github.com/patte/tower-sessions-rusqlite-store/pull/28

# 0.14.0

- use bundled sqlite for examples and tests by @arthmis in https://github.com/patte/tower-sessions-rusqlite-store/pull/24
- update tower-sessions (0.14.0) and axum (0.8) dependencies by @arthmis in https://github.com/patte/tower-sessions-rusqlite-store/pull/23
- deps: update dependencies (thiserror v2.0.11, tower-cookies v0.11.0) by @patte in https://github.com/patte/tower-sessions-rusqlite-store/pull/25

All dependency changes:
`thiserror = "1.0.64" => "2.0.11"`
`axum = "0.7.7" => "0.8"`
`tower-sessions = "0.13.0" => "0.14.0"`
`tower-cookies = "0.10.0" => "0.11.0"`
`tokio-rusqlite = "0.6.0"`

# 0.13.0

- deps: bump dependencies (tower-sessions v0.13, tokio-rusqlite v0.6, rusqlite 0.32) by @patte in https://github.com/patte/tower-sessions-rusqlite-store/pull/13

# 0.12.0

- Update `tower-sessions` to `0.12.0` and implement `SessionStore::create`.
- add unit tests from [maxcountryman/tower-sessions/memory-store](https://github.com/maxcountryman/tower-sessions/blob/6ad8933b4f5e71f3202f0c1a28f194f3db5234c8/memory-store/src/lib.rs#L62)
- report test coverage to codecov

# 0.11.1-2

- Update `rusqlite` to `0.31.0`.

# 0.11.1

- Update `tower-sessions` to `0.11.1`.

# 0.1.0

- Initial release :tada: