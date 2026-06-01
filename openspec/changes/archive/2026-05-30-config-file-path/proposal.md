## Why

The schedule file path currently uses the `directories` crate's `ProjectDirs` which resolves to OS-specific locations like `~/Library/Application Support/com.example.nine-cron/schedules.toml` on macOS. This creates inconsistency with the README documentation which shows `~/.config/nine-cron/schedules.toml`. The user wants a simpler, cross-platform config path at `$HOME/.config/nine-cron/schedulers.toml`.

## What Changes

- **BREAKING**: Change schedule file path from `~/Library/Application Support/com.example.nine-cron/schedules.toml` to `$HOME/.config/nine-cron/schedulers.toml`
- **BREAKING**: Rename file from `schedules.toml` to `schedulers.toml`
- Remove dependency on `directories` crate for config path (may still be used for data dir)
- Update README documentation to reflect new path

## Capabilities

### New Capabilities

- `config-path`: Defines where configuration files are stored on disk

### Modified Capabilities

(none - this is a path change, not a requirement change)

## Impact

- **Code**: `src/config.rs` - `config_path()` function
- **Dependencies**: May remove `directories` crate if no longer needed
- **Migration**: Users with existing schedules need to manually move `schedules.toml` to new location, or implement migration logic
- **Documentation**: `README.md` needs update
