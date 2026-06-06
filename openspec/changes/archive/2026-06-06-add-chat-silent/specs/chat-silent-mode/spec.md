## ADDED Requirements

### Requirement: Silent mode suppresses decorative output
The chat command SHALL accept a `--silent` (`-s`) flag that suppresses all decorative output when provided.

#### Scenario: Silent flag suppresses header
- **WHEN** user runs `nine-cron chat --silent --yes --msg "remind me at 3pm"`
- **THEN** the header box art and session info SHALL NOT be displayed

#### Scenario: Silent flag suppresses user message echo
- **WHEN** user runs `nine-cron chat --silent --yes --msg "remind me at 3pm"`
- **THEN** the "You:" message echo SHALL NOT be displayed

#### Scenario: Silent flag suppresses command preview
- **WHEN** user runs `nine-cron chat --silent --yes --msg "remind me at 3pm"`
- **THEN** the "Command:" preview SHALL NOT be displayed

#### Scenario: Silent flag suppresses AI clarification messages
- **WHEN** user runs `nine-cron chat --silent --msg "remind me"`
- **THEN** AI clarification questions SHALL NOT be displayed

### Requirement: Silent mode preserves success output
The chat command SHALL display the success message even when `--silent` is provided.

#### Scenario: Success message shown in silent mode
- **WHEN** user runs `nine-cron chat --silent --yes --msg "remind me at 3pm"`
- **THEN** the system SHALL display "Schedule added! Run at: {timestamp}"

### Requirement: Silent mode preserves error output
The chat command SHALL display error messages even when `--silent` is provided.

#### Scenario: Error shown in silent mode
- **WHEN** user runs `nine-cron chat --silent --msg "invalid request"`
- **AND** an error occurs during processing
- **THEN** the error message SHALL be displayed

### Requirement: Silent mode does not auto-confirm
The `--silent` flag SHALL NOT automatically skip the confirmation prompt.

#### Scenario: Confirmation prompt shown with silent only
- **WHEN** user runs `nine-cron chat --silent --msg "remind me at 3pm"`
- **AND** `-y` is not provided
- **THEN** the system SHALL display "Execute? (y/N):" prompt

#### Scenario: Silent with yes skips confirmation
- **WHEN** user runs `nine-cron chat --silent --yes --msg "remind me at 3pm"`
- **THEN** the confirmation prompt SHALL be skipped
- **AND** the schedule SHALL be created automatically
