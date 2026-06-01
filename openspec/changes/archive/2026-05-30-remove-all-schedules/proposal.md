## Why

Currently, removing schedules requires specifying each ID individually with `nine-cron schedule remove <id>`. For bulk cleanup, users must remove schedules one by one. A `--all` flag with confirmation prompt provides a convenient way to clear all schedules at once.

## What Changes

- Add `--all` flag to `nine-cron schedule remove` to remove all schedules
- Add `-y` flag to skip confirmation prompt when using `--all`
- Without `-y`, prompt user to confirm with "y" before deleting

## Capabilities

### New Capabilities

- `remove-all-schedules`: Remove all schedule items with confirmation prompt

### Modified Capabilities

(none - this extends existing remove functionality, not a requirement change)

## Impact

- **Code**: `src/main.rs` - `ScheduleAction::Remove` variant and its handler
- **CLI**: New flags `--all` and `-y` on `schedule remove` subcommand
