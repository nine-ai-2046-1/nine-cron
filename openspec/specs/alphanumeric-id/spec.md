### Requirement: Alphanumeric ID generation
The system SHALL generate IDs using only alphanumeric characters (a-z, A-Z, 0-9) without hyphens or other special characters.

#### Scenario: Schedule entry ID
- **WHEN** a new schedule entry is created
- **THEN** the ID SHALL contain only alphanumeric characters

#### Scenario: Run ID
- **WHEN** a new run is created
- **THEN** the ID SHALL contain only alphanumeric characters

### Requirement: ID length
The system SHALL generate IDs of 32 characters (UUID without hyphens).

#### Scenario: Schedule entry ID length
- **WHEN** a new schedule entry is created
- **THEN** the ID SHALL be exactly 32 characters long

#### Scenario: Run ID length
- **WHEN** a new run is created
- **THEN** the ID SHALL be exactly 32 characters long
