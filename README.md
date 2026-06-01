# nine-cron 🕘✨


Features
- Run any command and stream logs as NLJSON (one JSON object per stdout/stderr line, final metadata line).
- Schedule one-off or recurring tasks with flexible recurrence tokens (s,m,h,d,w,mo) and relative offsets (+6m, +1h30m).
- Run a scheduler daemon to auto-trigger due tasks.
- Persist schedules in a TOML file under your OS config directory.
- Schedule IDs are alphanumeric (no hyphens) for easy copy/paste.

When To Use

- Reminder / agent notifications: when an agent needs to remind a user at a specific time, use nine-cron schedule to register the reminder command. Example: the agent can call the CLI to schedule a reminder that will execute an external notifier like `opencb`:

```bash
./target/release/nine-cron schedule add -T "Pay rent reminder" +1h -- "opencb send 'this is msg to user for reminding sth'"
```

If the reminder should repeat (user explicitly requests recurrence), include `-r "1d"` (or another recurrence token). If the user does not specify recurrence but the agent thinks the reminder is likely repetitive, the agent should ask the user whether to set a recurrence before calling nine-cron.

To inspect scheduled tasks, use:

```bash
./target/release/nine-cron schedule list
```
which prints a human-readable table by default.

Installation

1. Build from source:

```bash
cargo build --release
```

2. The release binary is at `target/release/nine-cron`.

Quickstart

 - Run a command now (quote the whole command if it contains spaces or shell characters):

```bash
./target/release/nine-cron run "echo hello"
```

 - Add a one-off schedule (run after 6 seconds). Quote command when it contains spaces or shell features:

```bash
./target/release/nine-cron schedule add +6s "echo hello"
```

- Add a recurring schedule (every day at 12:00 HKT):

```bash
./target/release/nine-cron schedule add -t 12:00 -r "1d" echo daily-job
```

 - Start the daemon (poll every 10s by default). Use -i to change poll interval in seconds:

```bash
./target/release/nine-cron daemon -i 5
```

 - Remove a schedule by ID:

```bash
./target/release/nine-cron schedule remove <id>
```

 - Remove all schedules (with confirmation):

```bash
./target/release/nine-cron schedule remove --all
```

 - Remove all schedules without confirmation:

```bash
./target/release/nine-cron schedule remove --all -y
```

Daemon behaviour and logs

- The daemon runs a polling loop and executes scheduled jobs when due. It does not continuously print per-job logs to stdout. This avoids noisy console output when running as a background service.
- Per-run metadata (and simple log file) is stored in your OS data directory under `runs/`. Example on Linux: `~/.local/share/nine-cron/runs/<run_id>.log`.
- When you add a schedule with `nine-cron schedule add`, the CLI prints `schedule added` on success.

NLJSON output

The CLI streams structured log lines as NLJSON. Each stdout/stderr line is emitted as a JSON object with run_id, stream, text and timestamp. When the run finishes, the CLI emits a final metadata JSON line with start_time, end_time and exit_code. This makes it easy to pipe logs into processing tools.

Config and runs location

- Schedules stored in: `~/.config/nine-cron/schedulers.toml` (all platforms)
- Per-run logs stored in the data dir under `runs/` (example: `~/.local/share/nine-cron/runs/<run_id>.log`).

Contributing

If you want to extend the project, follow the repository style and open a branch named `dev/<task-name>`.
