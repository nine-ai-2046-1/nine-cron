// Entrypoint moved from crates to src
use clap::{Parser, Subcommand};
use tracing_subscriber;

use nine_cron::{run_with_runner, SystemRunner};
use nine_cron::config; // access config functions
use chrono::Utc;
mod schedule_utils;
pub mod chat;

#[derive(Parser)]
#[command(name = "nine-cron", about = "nine-cron: local CLI scheduler. Use quoted command strings when the command contains spaces or shell characters (e.g. redirection, pipes). Examples: nine-cron run \"date > test.log\" ")]
struct Cli { #[command(subcommand)] command: Commands }

#[derive(Subcommand)]
enum Commands { 
    /// Run a command immediately. Provide the whole command as a single quoted string when it contains shell syntax.
    Run { #[arg(required = true, help = "command to run; quote the entire command if it contains spaces or shell special characters")] cmd: String },
    /// Manage schedules: add, list, remove
    Schedule { #[command(subcommand)] action: ScheduleAction },
    /// Run as a daemon to execute scheduled jobs.
    /// -i sets loop interval in seconds. --catch-up enables executing missed runs on startup (use with care).
    Daemon { #[arg(short = 'i', long = "interval", help = "daemon loop interval in seconds")] interval: Option<u64>, #[arg(long = "catch-up", help = "execute missed schedules on startup (may cause many runs)")] catch_up: bool, #[arg(long = "max-catch-up", help = "maximum number of missed occurrences to run per schedule when catch-up is enabled", default_value_t = 100u32)] max_catch_up: u32 },
    /// Chat with AI to create schedules using natural language
    Chat { #[arg(short = 't', long = "title", help = "unique session title for the chat (auto-generated if not provided)")] title: Option<String>, #[arg(short = 'm', long = "msg", help = "your message to the AI", required_unless_present = "interactive")] msg: Option<String>, #[arg(short = 'i', long = "interactive", help = "start interactive multi-turn mode")] interactive: bool, #[arg(short = 'y', long = "yes", help = "auto accept and execute without confirmation")] yes: bool, #[arg(short = 's', long = "silent", help = "suppress decorative output, show only success/error messages")] silent: bool }
}

#[derive(Subcommand)]
enum ScheduleAction {
    // Positional args: [<relative_or_time>] <CMD>
    // We accept a variable number of positional args so callers can use shorthand like: schedule add +6s "echo 'hi'"
    Add { #[arg(short = 't', help = "absolute time to run (HH:MM)")] time: Option<String>, #[arg(short = 'd', help = "specific date to run (YYYY-MM-DD)")] date: Option<String>, #[arg(short = 'r', num_args = 1.., help = "relative durations or recurrence tokens (e.g. +6s, 1d)")] recurrence: Vec<String>, #[arg(short = 'T', long = "title", help = "title for the scheduled task", required=true)] title: String, #[arg(required = true, num_args = 1.., help = "positional arguments: optionally a relative token (e.g. +6s) followed by the command string")] args: Vec<String>, #[arg(short = 'n', help = "note for this schedule")] note: Option<String> },
    List { #[arg(long = "json", help = "output JSONL instead of a human table")] json: bool },
    /// Search schedules by keyword in title or command. If no query provided, enter interactive prompt.
    Search { #[arg(help = "query string to search for", required = false)] query: Option<String> },
    Remove { id: Option<String>, #[arg(long = "all", help = "remove all schedules")] all: bool, #[arg(short = 'y', long = "yes", help = "skip confirmation prompt")] yes: bool },
}

fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { cmd } => {
            let args = vec!["run".to_string(), cmd];
            let runner = SystemRunner;
            let code = run_with_runner(&runner, &args);
            std::process::exit(code);
        }

        Commands::Schedule { action } => match action {
            ScheduleAction::Add { time, date, recurrence, title, args, note } => {
                if args.is_empty() { eprintln!("no command provided"); std::process::exit(2); }
                let cmd: String;
                let mut initial_run: Option<chrono::DateTime<Utc>> = None;

                // positional relative token
                if args.len() >= 2 && (args[0].starts_with('+') || args[0].starts_with('-')) {
                    if let Some(d) = schedule_utils::parse_duration_token(&args[0]) {
                        initial_run = Some(Utc::now() + d);
                    }
                    cmd = args[1..].join(" ");
                } else {
                    cmd = args.join(" ");
                }

                // if time/date provided and initial_run not set, parse it
                if initial_run.is_none() {
                    if time.is_some() || date.is_some() {
                        match schedule_utils::parse_date_time(date.as_deref(), time.as_deref()) {
                            Ok(dt) => initial_run = Some(dt),
                            Err(e) => { eprintln!("failed to parse date/time: {:?}", e); std::process::exit(1); }
                        }
                    }
                }

                // if still none, and recurrence provided, use recurrence as initial delay
                if initial_run.is_none() && !recurrence.is_empty() {
                    if let Some(d) = schedule_utils::parse_duration_token(&recurrence[0]) {
                        initial_run = Some(Utc::now() + d);
                    }
                }

                // fallback to now
                let run_at = initial_run.unwrap_or_else(|| Utc::now());
                let recur = if !recurrence.is_empty() { Some(recurrence.join(" ")) } else { None };

                let mut file = match config::load_schedules() {
                    Ok(f) => f,
                    Err(e) => { eprintln!("failed to load schedules: {:?}", e); std::process::exit(1); }
                };
                    let sanitized_title = schedule_utils::sanitize_title(&title);
                    if sanitized_title.is_empty() {
                        eprintln!("invalid title: title empty after sanitization. Provide -T/--title with alphanumeric characters");
                        std::process::exit(2);
                    }
                    let entry = config::ScheduleEntry {
                        id: config::generate_id(),
                        title: sanitized_title,
                        cmd: cmd.clone(),
                        run_at,
                        recurrence: recur,
                        created_at: Utc::now(),
                        note: note.clone(),
                    };
                file.schedules.push(entry);
                if let Err(e) = config::save_schedules(&file) {
                    eprintln!("failed to save schedule: {:?}", e);
                    std::process::exit(1);
                }
                    // Inform user with HKT timestamp so times are always easy to read for HK users
                    let run_at_hkt = run_at.with_timezone(&chrono_tz::Asia::Hong_Kong).format("%Y-%m-%d %H:%M:%S %Z");
                    println!("schedule added (run_at: {})", run_at_hkt);
                    std::process::exit(0);
            }

            ScheduleAction::List { json } => {
                match config::load_schedules() {
                    Ok(file) => {
                        if file.schedules.is_empty() {
                            println!("no schedules");
                        } else {
                            if json {
                                for s in file.schedules.iter() {
                                    // present run_at / created_at in HKT for human friendliness
                                    let run_at_hkt = s.run_at.with_timezone(&chrono_tz::Asia::Hong_Kong).to_rfc3339();
                                    let created_hkt = s.created_at.with_timezone(&chrono_tz::Asia::Hong_Kong).to_rfc3339();
                                    let obj = serde_json::json!({"id": s.id, "title": s.title, "cmd": s.cmd, "run_at": run_at_hkt, "recurrence": s.recurrence, "note": s.note, "created_at": created_hkt});
                                    println!("{}", obj.to_string());
                                }
                            } else {
                                // human table including cmd (truncated)
                                println!("{:<36}  {:<20}  {:<20}  {:<10}  {}", "ID", "TITLE", "RUN_AT", "RECURRENCE", "CMD");
                                for s in file.schedules.iter() {
                                    // Safe truncation for multi-byte characters
                                    let cmd_display: String = s.cmd.chars().take(40).collect();
                                    let cmd_display = if cmd_display.len() < s.cmd.len() { format!("{}...", cmd_display) } else { cmd_display };
                                        let run_at_hkt = s.run_at.with_timezone(&chrono_tz::Asia::Hong_Kong).format("%Y-%m-%d %H:%M:%S %Z");
                                        println!("{:<36}  {:<20}  {:<20}  {:<10}  {}", s.id, s.title, run_at_hkt, s.recurrence.clone().unwrap_or_default(), cmd_display);
                                }
                            }
                        }
                        std::process::exit(0);
                    }
                    Err(e) => { eprintln!("failed to load schedules: {:?}", e); std::process::exit(1); }
                }
            }

            ScheduleAction::Remove { id, all, yes } => {
                if all {
                    match config::load_schedules() {
                        Ok(file) => {
                            if file.schedules.is_empty() {
                                println!("no schedules to remove");
                                std::process::exit(0);
                            }
                            if !yes {
                                use std::io::{stdin, stdout, Write};
                                print!("Remove all schedules? (y/N): ");
                                let _ = stdout().flush();
                                let mut input = String::new();
                                let _ = stdin().read_line(&mut input);
                                if input.trim().to_lowercase() != "y" {
                                    println!("cancelled");
                                    std::process::exit(0);
                                }
                            }
                            let empty = config::ScheduleFile::default();
                            if let Err(e) = config::save_schedules(&empty) {
                                eprintln!("failed to save schedules: {:?}", e);
                                std::process::exit(1);
                            }
                            println!("removed all schedules");
                            std::process::exit(0);
                        }
                        Err(e) => { eprintln!("failed to load schedules: {:?}", e); std::process::exit(1); }
                    }
                } else {
                    let id = match id {
                        Some(id) => id,
                        None => { eprintln!("no schedule id provided (use --all to remove all)"); std::process::exit(2); }
                    };
                    match config::load_schedules() {
                        Ok(mut file) => {
                            let before = file.schedules.len();
                            file.schedules.retain(|s| s.id != id);
                            let after = file.schedules.len();
                            if let Err(e) = config::save_schedules(&file) { eprintln!("failed to save schedules: {:?}", e); std::process::exit(1); }
                            if after < before { println!("removed {}", id); std::process::exit(0); } else { println!("no schedule with id {}", id); std::process::exit(2); }
                        }
                        Err(e) => { eprintln!("failed to load schedules: {:?}", e); std::process::exit(1); }
                    }
                }
            }

            ScheduleAction::Search { query } => {
                // load schedules and filter by query (in title or cmd)
                let q = if let Some(q) = query { q } else {
                    // interactive prompt
                    use std::io::{stdin,stdout,Write};
                    print!("search query: "); let _ = stdout().flush(); let mut input = String::new(); let _ = stdin().read_line(&mut input); input.trim().to_string()
                };
                if q.is_empty() { println!("empty query"); std::process::exit(2); }
                match config::load_schedules() {
                    Ok(file) => {
                        let ql = q.to_lowercase();
                        let mut found = 0;
                        println!("{:<36}  {:<20}  {:<20}  {:<10}", "ID", "TITLE", "RUN_AT", "RECURRENCE");
                        for s in file.schedules.iter() {
                            if s.title.to_lowercase().contains(&ql) || s.cmd.to_lowercase().contains(&ql) {
                                let run_at_hkt = s.run_at.with_timezone(&chrono_tz::Asia::Hong_Kong).format("%Y-%m-%d %H:%M:%S %Z");
                                println!("{:<36}  {:<20}  {:<20}  {:<10}", s.id, s.title, run_at_hkt, s.recurrence.clone().unwrap_or_default());
                                found += 1;
                            }
                        }
                        if found == 0 { println!("no matches for '{}'", q); }
                        std::process::exit(0);
                    }
                    Err(e) => { eprintln!("failed to load schedules: {:?}", e); std::process::exit(1); }
                }
            }
        },

        Commands::Chat { title, msg, interactive, yes, silent } => {
            let msg_str = msg.unwrap_or_default();
            if let Err(e) = chat::run_chat(title.as_deref(), &msg_str, interactive, yes, silent) {
                eprintln!("chat error: {:?}", e);
                std::process::exit(1);
            }
            std::process::exit(0);
        }

        Commands::Daemon { interval, catch_up, max_catch_up } => {
            let it = interval.unwrap_or(10);
            if let Err(e) = run_daemon(it, catch_up, max_catch_up) { eprintln!("daemon error: {:?}", e); }
            std::process::exit(0);
        }
    }
}

fn run_daemon(interval: u64, catch_up: bool, max_catch_up: u32) -> anyhow::Result<()> {
    println!("nine-cron daemon starting; interval={}s, catch_up={}", interval, catch_up);

    // Normalization step on startup: decide what to do with missed schedules
    let now = Utc::now();
    let mut file = match config::load_schedules() {
        Ok(f) => f,
        Err(e) => { eprintln!("failed to load schedules on startup: {:?}", e); return Err(e); }
    };

    if !file.schedules.is_empty() {
        let mut changed = false;
        for s in file.schedules.clone().iter_mut() {
            if s.run_at <= now {
                if let Some(recur) = &s.recurrence {
                    if let Some(rec_dur) = schedule_utils::parse_duration_token(recur) {
                        let elapsed = now.signed_duration_since(s.run_at);
                        let rec_secs = rec_dur.num_seconds();
                        let elapsed_secs = elapsed.num_seconds();
                        let missed = (elapsed_secs / rec_secs) + 1; // occurrences passed
                        if catch_up {
                            let to_run = std::cmp::min(missed as u32, max_catch_up);
                            for _ in 0..to_run {
                                println!("catch-up executing schedule id={} cmd={}", s.id, s.cmd);
                                let runner = SystemRunner;
                                let args = vec!["run".to_string(), s.cmd.clone()];
                                let _ = run_with_runner(&runner, &args);
                            }
                            // advance run_at by missed occurrences (cap by missed)
                            let advance_secs = (missed as i64) * rec_secs;
                            for item in file.schedules.iter_mut() { if item.id == s.id { item.run_at = item.run_at + chrono::Duration::seconds(advance_secs); } }
                            changed = true;
                        } else {
                            // advance to next future occurrence without executing
                            let k = (elapsed_secs / rec_secs) + 1;
                            let advance_secs = (k as i64) * rec_secs;
                            for item in file.schedules.iter_mut() { if item.id == s.id { item.run_at = item.run_at + chrono::Duration::seconds(advance_secs); } }
                            changed = true;
                        }
                    } else {
                        eprintln!("cannot parse recurrence {} for id={}", recur, s.id);
                    }
                } else {
                    // one-off missed
                    if catch_up {
                        println!("catch-up executing one-off schedule id={} cmd={}", s.id, s.cmd);
                        let runner = SystemRunner;
                        let args = vec!["run".to_string(), s.cmd.clone()];
                        let _ = run_with_runner(&runner, &args);
                        // remove after executing
                        file.schedules.retain(|x| x.id != s.id);
                        changed = true;
                    } else {
                        // drop missed one-off silently (but print message)
                        println!("dropping missed one-off schedule id={} cmd={}", s.id, s.cmd);
                        file.schedules.retain(|x| x.id != s.id);
                        changed = true;
                    }
                }
            }
        }
        if changed { if let Err(e) = config::save_schedules(&file) { eprintln!("failed to save schedules during normalization: {:?}", e); } }
    }

    // main loop
    // track last verbose sleep print to reduce noise
    let mut last_verbose = Utc::now() - chrono::Duration::minutes(30);
    loop {
        // load schedules
        let mut file = match config::load_schedules() {
            Ok(f) => f,
            Err(e) => { eprintln!("failed to load schedules: {:?}", e); std::thread::sleep(std::time::Duration::from_secs(interval)); continue; }
        };
        let now = Utc::now();
        // collect due schedules
        let due: Vec<_> = file.schedules.iter().filter(|s| s.run_at <= now).cloned().collect();
        if due.is_empty() {
            // only print verbose status every 15 minutes
            if Utc::now().signed_duration_since(last_verbose) > chrono::Duration::minutes(15) {
                println!("no schedules due; sleeping {}s", interval);
                last_verbose = Utc::now();
            }
        } else {
            println!("found {} schedule(s) due", due.len());
            for s in due {
                println!("executing schedule id={} cmd={}", s.id, s.cmd);
                let runner = SystemRunner;
                let args = vec!["run".to_string(), s.cmd.clone()];
                let code = run_with_runner(&runner, &args);
                println!("schedule id={} exit_code={}", s.id, code);
                if let Some(recur) = &s.recurrence {
                    // compute next run and update
                    if let Some(next) = schedule_utils::compute_next_run_at(s.run_at, recur) {
                        for item in file.schedules.iter_mut() {
                            if item.id == s.id { item.run_at = next; }
                        }
                    } else {
                        // cannot compute next -> remove
                        file.schedules.retain(|x| x.id != s.id);
                    }
                } else {
                    // one-off -> remove
                    file.schedules.retain(|x| x.id != s.id);
                }
            }
            // save changed schedules
            if let Err(e) = config::save_schedules(&file) { eprintln!("failed to save schedules: {:?}", e); }
        }
        std::thread::sleep(std::time::Duration::from_secs(interval));
    }
}
