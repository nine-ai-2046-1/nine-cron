## ADDED Requirements

### Requirement: Schedule file path
The system SHALL store the schedule file at `$HOME/.config/nine-cron/schedulers.toml`.

#### Scenario: Config path on macOS
- **WHEN** the application runs on macOS
- **THEN** the schedule file path SHALL be `$HOME/.config/nine-cron/schedulers.toml` (not `~/Library/Application Support/com.example.nine-cron/schedules.toml`)

#### Scenario: Config path on Linux
- **WHEN** the application runs on Linux
- **THEN** the schedule file path SHALL be `$HOME/.config/nine-cron/schedulers.toml`

#### Scenario: Config path on Windows
- **WHEN** the application runs on Windows
- **THEN** the schedule file path SHALL be `$HOME/.config/nine-cron/schedulers.toml`

### Requirement: Config directory creation
The system SHALL create the `$HOME/.config/nine-cron/` directory if it does not exist.

#### Scenario: Directory does not exist
- **WHEN** the application starts and `$HOME/.config/nine-cron/` does not exist
- **THEN** the system SHALL create the directory before writing the schedule file

### Requirement: File name
The schedule file SHALL be named `schedulers.toml` (not `schedules.toml`).

#### Scenario: File name consistency
- **WHEN** the application reads or writes the schedule file
- **THEN** the file name SHALL be `schedulers.toml`
