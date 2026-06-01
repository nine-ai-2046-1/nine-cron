## 1. CLI Definition

- [x] 1.1 Update `ScheduleAction::Remove` enum variant to add `--all` and `-y` flags
- [x] 1.2 Update `ScheduleAction::Remove` to make `id` optional when `--all` is used

## 2. Core Implementation

- [x] 2.1 Add confirmation prompt logic when `--all` is used without `-y`
- [x] 2.2 Implement remove-all logic to clear all schedules from file
- [x] 2.3 Handle empty schedule list case

## 3. Testing

- [x] 3.1 Build project with `cargo build --release` to verify changes compile
- [x] 3.2 Test `nine-cron schedule remove --all -y` removes all schedules
