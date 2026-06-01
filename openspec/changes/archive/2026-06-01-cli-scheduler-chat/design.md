## Context

The `nine-cron` CLI currently requires users to know exact syntax for scheduling commands. Users think in natural language ("run backup every night at 2am") but must translate to `nine-cron schedule add -t 02:00 -r 1d -T "backup" "backup_script.sh"`. This creates friction and errors.

The project has access to `nine-poe`, a CLI tool that invokes AI models. By combining `nine-cron` with `nine-poe`, we can create a chat interface that understands natural language and produces valid schedule commands.

Current state: `nine-cron` has `run`, `schedule`, and `daemon` subcommands. The `schedule add` command accepts `-t` (time), `-d` (date), `-r` (recurrence), `-T` (title), and positional command arguments.

## Goals / Non-Goals

**Goals:**
- Natural language input → valid `nine-cron schedule add` command
- Multi-turn conversation for clarification and confirmation
- Session persistence via `--session` parameter
- Validate AI output before execution
- Handle ambiguous requests by asking follow-up questions

**Non-Goals:**
- Modifying the underlying scheduling engine
- Supporting AI models other than NineGemini3Flash (can be extended later)
- Real-time streaming of AI responses
- Offline operation (requires `nine-poe` and AI backend)

## Decisions

### 1. Shell out to `nine-poe` via `std::process::Command`

**Decision**: Use `std::process::Command` to invoke `nine-poe` rather than importing a library.

**Rationale**: `nine-poe` is a separate CLI tool with its own dependency tree. Shell execution keeps `nine-cron` dependencies minimal and avoids version conflicts. The overhead of process spawning is negligible for an interactive chat use case.

**Alternatives considered**:
- Link `nine-poe` as a library: Rejected due to coupling and dependency bloat
- HTTP API call: `nine-poe` doesn't expose an HTTP interface

### 2. System prompt design with structured output

**Decision**: Design a system prompt that instructs the AI to return JSON with fields: `action`, `params`, `needs_clarification`, `clarification_question`.

**Rationale**: Structured output is easier to parse and validate than free-form text. JSON allows programmatic extraction of the schedule command parameters. The `needs_clarification` flag enables multi-turn flow.

**Alternatives considered**:
- Return raw command string: Harder to validate, prone to parsing errors
- Return YAML/TOML: JSON is natively supported in Rust via `serde_json`

### 3. Session management via `--session` flag

**Decision**: Pass `--session "<title>"` to `nine-poe` to maintain conversation context across turns.

**Rationale**: `nine-poe`'s `--session` flag persists conversation history. Using the user-provided `--title` as session ID ensures context continuity for the same scheduling task.

**Alternatives considered**:
- Manage conversation history in Rust: Duplicates effort, `nine-poe` already handles this
- Use a random session ID: Loses context between invocations

### 4. Validation before execution

**Decision**: Parse AI response JSON, validate all fields, then construct and optionally display the `schedule add` command for user confirmation before executing.

**Rationale**: AI outputs must be validated to prevent invalid commands or injection. Showing the command before execution gives users control and builds trust.

**Alternatives considered**:
- Execute immediately without confirmation: Risky for destructive/unexpected commands
- Skip validation: Could pass malformed arguments

### 5. Confirmation flow

**Decision**: When AI returns `needs_clarification: false` and valid params, display the constructed command and prompt "Execute? (y/N)". User can also type "modify" to provide corrections.

**Rationale**: Standard chatbot UX pattern. Gives users final say before schedule creation.

### 6. Configurable model ID

**Decision**: Add `[chat]` section to `~/.config/nine-cron/schedulers.toml` with `model` field. If section is missing, create it with `model = "NineGemini3Flash"` as default.

**Rationale**: Users may want to switch AI models (e.g., for cost, speed, or capability reasons). Config file is already the project's configuration mechanism.

**Alternatives considered**:
- CLI flag `--model`: Duplicates config, adds clutter; config is more persistent
- Environment variable: Less discoverable, harder to document

## Risks / Trade-offs

- **[Risk]** `nine-poe` not installed or not in PATH → **Mitigation**: Check for `nine-poe` existence at startup, return clear error message with installation instructions
- **[Risk]** AI returns invalid/unexpected JSON → **Mitigation**: Robust parsing with fallback to display raw response; retry with reformatted prompt
- **[Risk]** AI misunderstands scheduling intent → **Mitigation**: Confirmation step allows user to reject and rephrase; session context enables correction
- **[Risk]** Session context grows unbounded → **Mitigation**: `nine-poe` manages session lifecycle; document that sessions can be cleared
- **[Trade-off]** Adds external dependency (`nine-poe`) → Accepted because AI capability is core value proposition
- **[Trade-off]** Synchronous execution blocks on AI response → Accepted for simplicity; streaming can be added later
