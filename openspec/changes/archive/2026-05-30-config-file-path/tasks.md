## 1. Core Implementation

- [x] 1.1 Modify `src/config.rs` `config_path()` to use `$HOME/.config/nine-cron/schedulers.toml` instead of `ProjectDirs`
- [x] 1.2 Update `runs_dir()` function if `directories` crate is removed (or keep it for data dir only)
- [x] 1.3 Remove `directories` crate from `Cargo.toml` if no longer needed

## 2. Documentation

- [x] 2.1 Update `README.md` to reflect new config path (`~/.config/nine-cron/schedulers.toml`)
- [x] 2.2 Update any error messages in `src/config.rs` to mention new file location

## 3. Testing

- [x] 3.1 Build project with `cargo build --release` to verify changes compile
- [x] 3.2 Run existing tests to ensure no regressions
