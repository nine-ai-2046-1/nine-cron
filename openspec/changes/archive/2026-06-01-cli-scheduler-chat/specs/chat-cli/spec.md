## ADDED Requirements

### Requirement: Chat command accepts title and message
The system SHALL provide a `chat` subcommand that accepts `--title` (required, unique session identifier) and `--msg` (required, user's natural language message) parameters.

#### Scenario: Valid chat invocation
- **WHEN** user runs `nine-cron chat --title "my-task" --msg "remind me to call mom every Tuesday at 9am"`
- **THEN** system processes the message and initiates AI conversation

#### Scenario: Missing title parameter
- **WHEN** user runs `nine-cron chat --msg "do something"`
- **THEN** system displays error: "the following required arguments were not provided: --title"

#### Scenario: Missing message parameter
- **WHEN** user runs `nine-cron chat --title "my-task"`
- **THEN** system displays error: "the following required arguments were not provided: --msg"

### Requirement: Configurable AI model ID
The system SHALL read the model ID from `[chat]` section in `~/.config/nine-cron/schedulers.toml`. If the `[chat]` section is missing, the system SHALL create it with `model = "NineGemini3Flash"` as default.

#### Scenario: Default model ID
- **WHEN** no `[chat]` section exists in config file
- **THEN** system uses `"NineGemini3Flash"` as model ID

#### Scenario: Custom model ID
- **WHEN** config file contains `[chat]\nmodel = "gpt-4o"`
- **THEN** system uses `"gpt-4o"` as model ID in nine-poe invocations

### Requirement: AI integration via nine-poe
The system SHALL invoke `nine-poe --model "<model_id>" --prompt "<system_prompt>\n\nUser: <msg>" --session "<title>"` to process natural language input.

#### Scenario: AI returns valid schedule parameters
- **WHEN** user provides a clear scheduling request (e.g., "run backup every night at 2am")
- **THEN** system calls nine-poe and parses JSON response containing action, params, needs_clarification, and clarification_question fields

#### Scenario: nine-poe not available
- **WHEN** system attempts to invoke nine-poe and the command is not found
- **THEN** system displays error: "nine-poe is required for chat feature. Install it from [link]"

#### Scenario: AI returns invalid JSON
- **WHEN** nine-poe returns non-JSON or malformed JSON response
- **THEN** system displays raw AI response and prompts user to retry

### Requirement: Multi-turn conversation support
The system SHALL maintain conversation context across multiple invocations using the `--session` parameter passed to nine-poe.

#### Scenario: Follow-up clarification
- **WHEN** AI responds with `needs_clarification: true` and a `clarification_question`
- **THEN** system displays the question to user and awaits next `--msg` input with same `--title`

#### Scenario: Session continuity
- **WHEN** user provides second message with same `--title`
- **THEN** nine-poe receives full conversation history from previous invocations

### Requirement: Command validation and confirmation
The system SHALL validate AI-generated schedule parameters and display the constructed command for user confirmation before execution.

#### Scenario: Valid schedule command
- **WHEN** AI returns `needs_clarification: false` with valid params (time, recurrence, title, command)
- **THEN** system displays: "I will run: nine-cron schedule add -t <time> -r <recurrence> -T <title> '<cmd>'\nExecute? (y/N)"

#### Scenario: User confirms execution
- **WHEN** system displays confirmation prompt and user types "y" or "yes"
- **THEN** system executes the schedule add command and displays result

#### Scenario: User declines execution
- **WHEN** system displays confirmation prompt and user types "n" or "N" or just presses Enter
- **THEN** system displays "Cancelled" and exits

#### Scenario: User requests modification
- **WHEN** system displays confirmation prompt and user types "modify" followed by corrections
- **THEN** system sends follow-up message to AI with original context and modification request

### Requirement: System prompt for schedule creation
The system SHALL use a system prompt that instructs the AI to: understand scheduling intent, extract time/date/recurrence/command, return structured JSON, and ask clarifying questions when information is insufficient.

#### Scenario: Ambiguous time specification
- **WHEN** user says "remind me tomorrow" without specifying time
- **THEN** AI returns `needs_clarification: true` with `clarification_question`: "What time tomorrow?"

#### Scenario: Complete schedule information
- **WHEN** user says "run /usr/local/bin/backup.sh every day at 3am"
- **THEN** AI returns `needs_clarification: false` with params: time="03:00", recurrence="1d", title derived from command, cmd="/usr/local/bin/backup.sh"
