## 1. Update CLI Arguments

- [x] 1.1 Make `--title` optional in `Commands::Chat` enum in `src/main.rs`
- [x] 1.2 Handle `Option<String>` for title in chat handler

## 2. Update AI Prompt

- [x] 2.1 Update system prompt to instruct AI to generate title when not provided
- [x] 2.2 Add title field to AI response JSON schema

## 3. Update Chat Logic

- [x] 3.1 When title is None, use AI-generated title as session ID
- [x] 3.2 Sanitize AI-generated title with `sanitize_title()`
- [x] 3.3 Fallback to message hash if AI doesn't return title

## 4. Testing

- [x] 4.1 Test with `--title` provided (existing behavior)
- [x] 4.2 Test without `--title` (auto-generate)
- [x] 4.3 Verify `cargo build --release` succeeds
