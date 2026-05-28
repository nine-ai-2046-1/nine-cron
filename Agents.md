# Agents Guide

This file instructs coding agents how to follow up on this project. Use the openspec artifacts under openspec/changes/cli-scheduler-for-cli to understand requirements and tasks.

Rules for the agent:

1. Always work on a branch named `dev/<task-name>`.
2. Run tests and build before proposing changes. Fix compile or test failures locally.
3. When making changes, keep them small and reversible. Use apply_patch for edits.
4. Update README.md and README-HK.md when CLI surface or examples change.
5. Use the openspec/*.md files as the source of truth for requirements and tasks.

Workflow:

- Read openspec/changes/cli-scheduler-for-cli/tasks.md to pick next task.
- Propose a change with explanation, then implement using apply_patch.
- Run `cargo build --release` and include build output in the change log.
