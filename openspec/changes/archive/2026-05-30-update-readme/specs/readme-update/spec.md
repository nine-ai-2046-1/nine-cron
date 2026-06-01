## ADDED Requirements

### Requirement: README documents config path
The README files SHALL document the config file location as `~/.config/nine-cron/schedulers.toml`.

#### Scenario: Config path in README
- **WHEN** user reads README.md or README-HK.md
- **THEN** documentation SHALL show `~/.config/nine-cron/schedulers.toml` as config location

### Requirement: README documents alphanumeric IDs
The README files SHALL document that schedule IDs are alphanumeric (no hyphens).

#### Scenario: ID format in README
- **WHEN** user reads README.md or README-HK.md
- **THEN** documentation SHALL mention alphanumeric schedule IDs

### Requirement: README documents remove-all feature
The README files SHALL document the `schedule remove --all` command with `-y` flag.

#### Scenario: Remove-all in README
- **WHEN** user reads README.md or README-HK.md
- **THEN** documentation SHALL show `nine-cron schedule remove --all` usage
