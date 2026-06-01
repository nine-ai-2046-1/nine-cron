## MODIFIED Requirements

### Requirement: Chat command accepts title and message
The system SHALL provide a `chat` subcommand that accepts `--title` (optional, unique session identifier) and `--msg` (required, user's natural language message) parameters. When `--title` is not provided, the system SHALL generate a valid title from the user's message.

#### Scenario: Valid chat invocation with title
- **WHEN** user runs `nine-cron chat --title "my-task" --msg "remind me to call mom every Tuesday at 9am"`
- **THEN** system processes the message and initiates AI conversation using provided title

#### Scenario: Valid chat invocation without title
- **WHEN** user runs `nine-cron chat --msg "remind me to call mom every Tuesday at 9am"`
- **THEN** system generates title from message and initiates AI conversation

#### Scenario: Missing message parameter
- **WHEN** user runs `nine-cron chat --title "my-task"`
- **THEN** system displays error: "the following required arguments were not provided: --msg"
