## 1. Setup and Dependencies

- [x] 1.1 Verify `nine-poe` availability at runtime (check PATH, return helpful error if missing)
- [x] 1.2 Create new module `src/chat.rs` for chat functionality
- [x] 1.3 Add `Chat` variant to `Commands` enum in `src/main.rs` with `--title` and `--msg` arguments
- [x] 1.4 Add `ChatConfig` struct with `model` field (default: "NineGemini3Flash") to `src/config.rs`
- [x] 1.5 Add `load_chat_config()` function to read `[chat]` section from `~/.config/nine-cron/schedulers.toml`, create with default `[chat]\nmodel = "NineGemini3Flash"` if missing

## 2. System Prompt Design

- [x] 2.1 Design system prompt that instructs AI to return JSON with `action`, `params`, `needs_clarification`, `clarification_question` fields
- [x] 2.2 Include schedule parameter extraction rules (time format HH:MM, recurrence tokens, title sanitization)
- [x] 2.3 Add examples of valid JSON responses for common scheduling patterns

## 3. AI Integration

- [x] 3.1 Implement `call_nine_poe(session: &str, msg: &str) -> Result<String>` function using `std::process::Command`
- [x] 3.2 Construct prompt with system prompt + user message for each invocation
- [x] 3.3 Pass `--session` parameter to maintain conversation context

## 4. Response Parsing

- [x] 4.1 Define `ChatResponse` struct with `action`, `params`, `needs_clarification`, `clarification_question` fields
- [x] 4.2 Implement JSON parsing with `serde_json` and proper error handling
- [x] 4.3 Handle malformed JSON gracefully (display raw response, prompt retry)

## 5. Confirmation Flow

- [x] 5.1 Display AI's clarification question when `needs_clarification: true`
- [x] 5.2 Construct `nine-cron schedule add` command string from parsed params
- [x] 5.3 Display constructed command and prompt "Execute? (y/N)"
- [x] 5.4 Handle user confirmation (y/yes), decline (n/no/Enter), or modify input

## 6. Command Execution

- [x] 6.1 Execute confirmed `schedule add` command via `run_with_runner`
- [x] 6.2 Display execution result (success with HKT timestamp, or error)
- [x] 6.3 Handle edge cases: invalid time, duplicate title, missing command

## 7. Testing and Validation

- [x] 7.1 Test with clear scheduling requests (e.g., "run backup every night at 2am")
- [x] 7.2 Test with ambiguous requests requiring clarification
- [x] 7.3 Test multi-turn conversation flow
- [x] 7.4 Test error handling: nine-poe missing, AI returns invalid JSON, user declines
- [x] 7.5 Verify `cargo build --release` succeeds with no warnings
