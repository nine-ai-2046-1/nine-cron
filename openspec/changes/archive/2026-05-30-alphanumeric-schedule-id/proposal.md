## Why

Schedule and run IDs currently use UUIDs with hyphens (e.g., `1c686711-acca-4b4d-954e-42d72bf00ba1`). Hyphens make IDs harder to copy/paste in terminals and scripts. The user wants alphanumeric-only IDs for easier use.

## What Changes

- **BREAKING**: Change schedule entry ID format from UUID with hyphens to alphanumeric-only string
- **BREAKING**: Change run ID format from UUID with hyphens to alphanumeric-only string
- Existing IDs on disk remain unchanged (no migration needed)

## Capabilities

### New Capabilities

- `alphanumeric-id`: Generate IDs using only alphanumeric characters (a-z, A-Z, 0-9)

### Modified Capabilities

(none - this is an ID format change, not a requirement change)

## Impact

- **Code**: `src/config.rs`, `src/main.rs`, `src/nine_cron.rs` - UUID generation
- **Dependencies**: `uuid` crate may still be needed, but `simple-alphanumeric-id` or similar could replace ID generation
- **Migration**: No migration needed - new IDs are generated on creation, old IDs remain valid
