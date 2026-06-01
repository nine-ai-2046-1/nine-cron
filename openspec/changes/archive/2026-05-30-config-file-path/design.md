## Context

The current implementation uses the `directories` crate's `ProjectDirs::from("com", "example", "nine-cron")` to determine config paths. On macOS this resolves to `~/Library/Application Support/com.example.nine-cron/schedules.toml`. The user wants to use `~/.config/nine-cron/schedulers.toml` instead, which is a simpler, more predictable path that works consistently across platforms.

## Goals / Non-Goals

**Goals:**
- Change config path to `$HOME/.config/nine-cron/schedulers.toml`
- Remove `directories` crate dependency if no longer needed for data dir
- Keep migration simple - document manual migration or provide helper

**Non-Goals:**
- Auto-migration from old path (keep it simple, manual migration)
- Changing the data dir path (`runs/` directory)
- Changing the TOML schema or data model

## Decisions

**Decision 1: Use `dirs::home_dir()` + `.config/nine-cron/schedulers.toml`**
- Rationale: Simple, predictable, cross-platform
- Alternatives considered:
  - `dirs::config_dir()` + `nine-cron/schedulers.toml`: On Linux this is `~/.config/`, but on macOS it's still `~/Library/Application Support/`
  - Keep `directories` crate: It's designed for this but produces OS-specific paths which the user doesn't want

**Decision 2: Keep filename as `schedulers.toml` (not `schedules.toml`)**
- Rationale: User explicitly requested this name
- This is a breaking change - existing users must rename their file

**Decision 3: Don't implement auto-migration**
- Rationale: Simpler, avoids complex path detection, users can easily move one file
- Trade-off: Slight inconvenience for existing users on first run

## Risks / Trade-offs

- **Risk**: Existing users lose access to their schedules on upgrade
  - **Mitigation**: Document migration in README, add error message pointing to new location

- **Risk**: `$HOME` might not be set in some environments
  - **Mitigation**: Fallback to `dirs::home_dir()` or error gracefully

- **Trade-off**: Removing `directories` crate reduces dependencies but loses automatic platform-specific data dir handling for `runs/` directory
  - **Mitigation**: Can keep `directories` for data dir only, or manually construct data dir path too
