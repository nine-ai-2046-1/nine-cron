## Why

Requiring `--title` for every chat invocation adds friction. Users should be able to just describe what they want and let the AI generate a valid title automatically. The title is mainly for session management and schedule identification, so auto-generating it from the user's intent makes the UX smoother.

## What Changes

- Make `--title` optional in the `chat` command
- When `--title` is not provided, AI generates a title from the user's message
- Title must be valid: alphanumeric and hyphens only, max 20 chars
- Use generated title for session ID and schedule title

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `chat-cli`: Title parameter becomes optional, AI generates title when not provided

## Impact

- Files modified: `src/main.rs`, `src/chat.rs`
- No breaking changes (title is still accepted, just not required)
