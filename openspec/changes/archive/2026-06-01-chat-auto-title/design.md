## Context

Currently `nine-cron chat --title "name" --msg "message"` requires both `--title` and `--msg`. Users often don't know what title to use until they see the AI's response. Making `--title` optional improves UX.

## Goals / Non-Goals

**Goals:**
- Make `--title` optional
- Auto-generate valid title from user's message via AI
- Use generated title for session ID and schedule

**Non-Goals:**
- Change title validation rules
- Modify existing title behavior when provided

## Decisions

**Approach:** Include title generation in the AI system prompt. When no title is provided:
1. First AI call returns both `title` and schedule params
2. Use generated title as session ID for subsequent calls
3. Fallback to hash of message if AI doesn't return title

**Alternative considered:** Generate title in Rust code - rejected because AI understands context better.

## Risks / Trade-offs

- [Risk] AI returns invalid title → Mitigation: Sanitize with existing `sanitize_title()` function
