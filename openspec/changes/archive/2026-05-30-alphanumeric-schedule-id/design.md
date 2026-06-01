## Context

The current implementation uses `Uuid::new_v4().to_string()` to generate IDs for schedule entries and runs. This produces strings like `1c686711-acca-4b4d-954e-42d72bf00ba1` with hyphens. The user wants alphanumeric-only IDs for easier copy/paste in terminals.

## Goals / Non-Goals

**Goals:**
- Generate IDs using only alphanumeric characters (a-z, A-Z, 0-9)
- Maintain sufficient uniqueness for practical use
- Keep the same ID length or shorter for readability

**Non-Goals:**
- Cryptographic randomness (not required for this use case)
- Backward compatibility with existing hyphenated IDs (old IDs remain valid)
- Changing the ID field type (remains String)

## Decisions

**Decision 1: Use UUID with hyphens removed**
- Rationale: Reuses existing `uuid` crate, minimal code change, no new dependencies
- Alternatives considered:
  - Custom alphanumeric generator: More code, same result
  - `nanoid` crate: Adds dependency for simple need

**Decision 2: Keep UUID length (36 chars without hyphens = 32 chars)**
- Rationale: Standard UUID v4 entropy maintained, just remove hyphens

## Risks / Trade-offs

- **Risk**: Existing hyphenated IDs on disk remain
  - **Mitigation**: No migration needed - old IDs work fine, new IDs are just easier to copy/paste
