## Why

Recent changes (config path, alphanumeric IDs, remove-all feature) are not documented in README files. Users need accurate documentation to understand the new features and file locations.

## What Changes

- Update `README.md` to document:
  - New config path: `~/.config/nine-cron/schedulers.toml`
  - Alphanumeric schedule IDs (no hyphens)
  - Remove all schedules feature (`--all` and `-y` flags)
- Update `README-HK.md` with same changes in Cantonese

## Capabilities

### New Capabilities

- `readme-update`: Documentation updates for recent feature changes

### Modified Capabilities

(none - documentation only, no requirement changes)

## Impact

- **Files**: `README.md`, `README-HK.md`
