## Context

The current `schedule remove` command only accepts a single ID. Users wanting to clear all schedules must run the command multiple times. This change adds `--all` flag with a confirmation prompt to safely remove all schedules at once.

## Goals / Non-Goals

**Goals:**
- Add `--all` flag to remove all schedules
- Add `-y` flag to skip confirmation prompt
- Prompt for confirmation by default when using `--all`

**Non-Goals:**
- Removing multiple specific IDs at once (not requested)
- Adding confirmation to single-ID remove (user explicitly chose which to remove)

## Decisions

**Decision 1: Use `--all` flag instead of special ID value**
- Rationale: More explicit, prevents accidental deletion, clearer CLI syntax
- Alternatives considered:
  - `nine-cron schedule remove all` as literal string: Could conflict with future ID formats
  - `nine-cron schedule remove --all`: Standard flag pattern, unambiguous

**Decision 2: Use `-y` for auto-confirm**
- Rationale: Common convention in CLI tools (e.g., `rm -f`, `apt -y`)
- Alternatives considered:
  - `--yes`: Longer but also common
  - `--force`: Usually means different thing (skip errors)

**Decision 3: Prompt reads from stdin**
- Rationale: Simple, works in terminals and scripts
- Alternatives considered:
  - Y/n prompt: Slightly more standard, but user requested "y" input

## Risks / Trade-offs

- **Risk**: User accidentally runs `--all` without `-y`
  - **Mitigation**: Confirmation prompt prevents accidental deletion

- **Risk**: Scripted usage might hang on prompt
  - **Mitigation**: `-y` flag allows non-interactive use
