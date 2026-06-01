## Why

Creating cron schedules requires users to understand the exact CLI syntax and parameters (`-t`, `-d`, `-r`, `-T`, `-n`). Users often want to describe *what* they want ("remind me to call mom every Tuesday at 9am") without knowing the exact command. An AI-powered chat interface can bridge this gap, translating natural language into valid schedule commands while supporting multi-turn clarification.

## What Changes

- Add new `nine-cron chat` subcommand with `--title` (unique session ID) and `--msg` (user's message) parameters
- Integrate with `nine-po` AI backend via `nine-poe --model "NineGemini3Flash"` for natural language understanding
- Support multi-turn conversations using `--session` parameter for context persistence
- AI returns validated `nine-cron schedule add` commands for user confirmation before execution
- AI can ask clarifying questions when intent is ambiguous (e.g., missing time, unclear recurrence)

## Capabilities

### New Capabilities
- `chat-cli`: AI-powered chat interface for natural language schedule creation with multi-turn conversation support

### Modified Capabilities

(none - this is a new addition)

## Impact

- **New CLI surface**: Adds `chat` subcommand to `nine-cron` binary
- **New dependency**: Requires `nine-poe` CLI tool to be available in PATH
- **File changes**: `src/main.rs` (new subcommand handler), new `src/chat.rs` module
- **User-facing**: Non-breaking addition; existing commands remain unchanged
