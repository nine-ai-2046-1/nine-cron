## Context

The `nine-cron chat` command currently outputs decorative elements (header box, session info, user message echo, command preview) that are useful for interactive use but clutter scripted/automated usage. Users need a minimal output mode.

Current output flow in `run_single()`:
1. `print_header()` - decorative box art
2. `print_user_msg()` - echoes user input
3. `process_message()` - may call `print_ai_msg()` for clarification
4. `print_command()` - shows the schedule command
5. `execute_schedule_add()` - prints success message

## Goals / Non-Goals

**Goals:**
- Add `--silent` flag that suppresses decorative output
- Keep success message: "Schedule added! Run at: {timestamp}"
- Keep error messages (always shown)
- Minimal code change, low risk

**Non-Goals:**
- Changing behavior of `--yes` flag
- Adding silent mode to other commands
- Changing error handling behavior
- Adding verbosity levels (verbose/normal/silent)

## Decisions

### Decision: Pass `silent` flag through function chain

**Choice**: Add `silent: bool` parameter to `run_chat()` → `run_single()` → `execute_schedule_add()`

**Alternatives considered:**
1. Global state/thread-local - rejected: adds complexity, harder to test
2. Struct config - rejected: overkill for single boolean
3. Environment variable - rejected: not CLI-friendly, harder to discover

**Rationale**: Simple parameter passing is idiomatic Rust, easy to trace, no side effects.

### Decision: Conditional print calls

**Choice**: Wrap decorative print calls in `if !silent { ... }` blocks

**Alternatives considered:**
1. Trait-based output abstraction - rejected: over-engineering for this scope
2. Macro-based approach - rejected: harder to read, no benefit for 5 call sites

**Rationale**: Direct conditionals are clear, minimal, and easy to remove later if needed.

### Decision: Success message stays in `execute_schedule_add()`

**Choice**: Keep the success print inside `execute_schedule_add()`, pass `silent` to skip only the decorative prints in `run_single()`

**Rationale**: The success message IS the output users want. Errors are already printed via `print_error()` which we keep.

## Risks / Trade-offs

- **Risk**: Forgetting to pass `silent` to a new print call → **Mitigation**: Simple code review, only 5-6 call sites
- **Trade-off**: Slightly more parameters in function signatures → acceptable for clarity
