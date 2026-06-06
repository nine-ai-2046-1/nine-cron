## Why

When using `nine-cron chat` in scripts or automation, the decorative output (header, session info, user message echo, command preview) clutters the result. Users need a minimal output mode that returns only the essential success message: "Schedule added! Run at: xxxx".

## What Changes

- Add `--silent` (`-s`) flag to the `chat` command
- When `--silent` is set, suppress all decorative output (header, user echo, AI messages, command preview)
- Keep the success message: "Schedule added! Run at: {timestamp}"
- Keep error messages (errors always display regardless of silent mode)
- `--silent` does NOT auto-skip confirmation prompt (use `-y` for that)
- Only applies to `chat` command, not other commands

## Capabilities

### New Capabilities

- `chat-silent-mode`: Minimal output mode for the chat command that suppresses decorative output while preserving success/error messages

### Modified Capabilities

None - this is a new feature, not a change to existing requirements.

## Impact

- **Code**: `src/main.rs` (CLI arg definition), `src/chat.rs` (output functions)
- **Dependencies**: None
- **APIs**: None
- **Breaking changes**: None - purely additive flag
