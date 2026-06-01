### Requirement: Remove all schedules with --all flag
The system SHALL support `--all` flag on `schedule remove` to remove all schedules.

#### Scenario: Remove all schedules with confirmation
- **WHEN** user runs `nine-cron schedule remove --all`
- **THEN** system SHALL prompt "Remove all schedules? (y/N): " and wait for input

#### Scenario: User confirms removal
- **WHEN** user enters "y" at the confirmation prompt
- **THEN** system SHALL remove all schedules and print "removed all schedules"

#### Scenario: User declines removal
- **WHEN** user enters anything other than "y" at the confirmation prompt
- **THEN** system SHALL not remove any schedules and print "cancelled"

### Requirement: Skip confirmation with -y flag
The system SHALL support `-y` flag to skip confirmation when using `--all`.

#### Scenario: Remove all schedules without prompt
- **WHEN** user runs `nine-cron schedule remove --all -y`
- **THEN** system SHALL remove all schedules immediately without prompting

#### Scenario: Remove all schedules with --yes
- **WHEN** user runs `nine-cron schedule remove --all --yes`
- **THEN** system SHALL remove all schedules immediately without prompting

### Requirement: No schedules to remove
The system SHALL handle empty schedule list gracefully.

#### Scenario: Remove all when no schedules exist
- **WHEN** user runs `nine-cron schedule remove --all -y` and no schedules exist
- **THEN** system SHALL print "no schedules to remove"
