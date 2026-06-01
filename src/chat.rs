use std::process::Command;
use std::io::{self, Write, BufRead, BufReader};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use std::fs;

use crate::config;
use crate::schedule_utils;

// ANSI color codes for pretty output
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const MAGENTA: &str = "\x1b[35m";
const BLUE: &str = "\x1b[34m";

fn intentions_dir() -> Result<std::path::PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| anyhow!("cannot determine home directory"))?;
    let dir = std::path::PathBuf::from(home)
        .join(".config")
        .join("nine-cron")
        .join("chats")
        .join("intentions");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn load_intentions() -> Result<String> {
    let dir = intentions_dir()?;
    let mut content = String::new();

    if dir.exists() {
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("md") {
                if let Ok(file_content) = fs::read_to_string(&path) {
                    content.push_str(&format!("\n--- Intent: {} ---\n", path.file_stem().unwrap().to_string_lossy()));
                    content.push_str(&file_content);
                    content.push('\n');
                }
            }
        }
    }

    if content.is_empty() {
        content = "\nNo intention files found. Create .md files in ~/.config/nine-cron/chats/intentions/\n".to_string();
    }

    Ok(content)
}

fn build_system_prompt() -> Result<String> {
    let intentions = load_intentions()?;
    let now = chrono::Local::now();
    let today = now.format("%Y-%m-%d").to_string();
    let current_time = now.format("%H:%M").to_string();

    Ok(format!(r#"You are a scheduling assistant for nine-cron. Your job is to understand user requests and create scheduled tasks.

## Current Date & Time
Today is: {today}
Current time: {current_time}

## Available Intention Patterns
{intentions}

## Response Format
Always respond with valid JSON only. No markdown, no explanation outside the JSON.

If user message matches an intention pattern:
```json
{{
  "action": "schedule_add",
  "needs_clarification": false,
  "clarification_question": null,
  "params": {{
    "time": "HH:MM",
    "date": "YYYY-MM-DD or null",
    "recurrence": "recurrence token or null",
    "title": "descriptive-title",
    "cmd": "command to run"
  }}
}}
```

If user message is ambiguous or missing info:
```json
{{
  "action": "schedule_add",
  "needs_clarification": true,
  "clarification_question": "What clarification is needed?",
  "params": null
}}
```

## Parameter Rules
- time: 24-hour format HH:MM
- recurrence: "Nd" (days), "Nh" (hours), "Nm" (minutes), null for one-time
- title: alphanumeric and hyphens only, max 20 chars, descriptive
- cmd: exact command to execute
- date: ALWAYS use the current year ({today_year}) when user mentions a date without year

## Relative Time Handling
When user says relative time like "after X minutes/hours/days", you MUST calculate the actual date and time based on current time.

Examples:
- Current time is 2026-06-01 15:00
- "remind me after 30 minutes" → time: "15:30", date: "2026-06-01"
- "remind me after 2 hours" → time: "17:00", date: "2026-06-01"
- "remind me after 3 days" → time: "15:00", date: "2026-06-04"
- "remind me tomorrow at 10am" → time: "10:00", date: "2026-06-02"
- "remind me next monday at 9am" → time: "09:00", date: "2026-06-08"

NEVER ask for the date/time when user provides relative time. Calculate it yourself.

## Intent Matching Priority
1. Calendar/reminder events → use opencb send with reminder message
2. General tasks → use the command provided
3. If no match → ask for clarification

Always respond with valid JSON only."#,
        today = today,
        today_year = now.format("%Y").to_string()
    ))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatResponse {
    pub action: String,
    pub needs_clarification: bool,
    pub clarification_question: Option<String>,
    pub params: Option<ScheduleParams>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScheduleParams {
    pub time: Option<String>,
    pub date: Option<String>,
    pub recurrence: Option<String>,
    pub title: String,
    pub cmd: String,
}

fn check_nine_poe_available() -> Result<()> {
    let output = Command::new("which")
        .arg("nine-poe")
        .output()
        .map_err(|_| anyhow!("nine-poe is required for chat feature. Install it from https://github.com/nine-poe/nine-poe"))?;

    if !output.status.success() {
        return Err(anyhow!("nine-poe is required for chat feature. Install it from https://github.com/nine-poe/nine-poe"));
    }
    Ok(())
}

fn call_nine_poe(model: &str, session: &str, msg: &str) -> Result<String> {
    check_nine_poe_available()?;

    let system_prompt = build_system_prompt()?;
    let prompt = format!("{}\n\nUser: {}", system_prompt, msg);

    let output = Command::new("nine-poe")
        .arg("--model")
        .arg(model)
        .arg("--prompt")
        .arg(&prompt)
        .arg("--session")
        .arg(session)
        .output()
        .map_err(|e| anyhow!("Failed to invoke nine-poe: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("nine-poe error: {}", stderr));
    }

    let response = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(response)
}

fn parse_chat_response(raw: &str) -> Result<ChatResponse> {
    let json_str = if let Some(start) = raw.find('{') {
        if let Some(end) = raw.rfind('}') {
            &raw[start..=end]
        } else {
            raw
        }
    } else {
        raw
    };

    serde_json::from_str(json_str)
        .map_err(|e| anyhow!("Failed to parse AI response as JSON: {}\nRaw response: {}", e, raw))
}

fn build_schedule_command(params: &ScheduleParams) -> String {
    let mut args = vec!["nine-cron".to_string(), "schedule".to_string(), "add".to_string()];

    if let Some(ref time) = params.time {
        args.push("-t".to_string());
        args.push(format!("\"{}\"", time));
    }

    if let Some(ref date) = params.date {
        args.push("-d".to_string());
        args.push(format!("\"{}\"", date));
    }

    if let Some(ref recurrence) = params.recurrence {
        args.push("-r".to_string());
        args.push(format!("\"{}\"", recurrence));
    }

    args.push("-T".to_string());
    args.push(format!("\"{}\"", params.title));

    // Quote the command and escape inner quotes
    let escaped_cmd = params.cmd.replace('"', "\\\"");
    args.push(format!("\"{}\"", escaped_cmd));

    args.join(" ")
}

fn print_header(title: &str) {
    println!();
    println!("{}{}╔══════════════════════════════════════════════════════════╗{}", CYAN, BOLD, RESET);
    println!("{}{}║  🗓️  nine-cron chat                                     ║{}", CYAN, BOLD, RESET);
    println!("{}{}╚══════════════════════════════════════════════════════════╝{}", CYAN, BOLD, RESET);
    println!();
    println!("  {}Session: {}{}{}", DIM, RESET, BOLD, title);
    println!("  {}Type 'quit' or 'exit' to end, 'help' for commands{}", DIM, RESET);
    println!();
}

fn print_user_msg(msg: &str) {
    println!("  {}👤 You:{}", GREEN, RESET);
    println!("  {}{}{}", GREEN, msg, RESET);
    println!();
}

fn print_ai_msg(msg: &str) {
    println!("  {}🤖 AI:{}", MAGENTA, RESET);
    for line in msg.lines() {
        println!("  {}{}{}", MAGENTA, line, RESET);
    }
    println!();
}

fn print_command(cmd: &str) {
    println!("  {}📋 Command:{}", BLUE, RESET);
    println!("  {}{}{}{}", BOLD, cmd, RESET, DIM);
    println!();
}

fn print_success(msg: &str) {
    println!("  {}✅ {}{}", GREEN, msg, RESET);
}

fn print_error(msg: &str) {
    println!("  {}❌ {}{}", RED, msg, RESET);
}

fn print_help() {
    println!();
    println!("  {}{}Commands:{}", BOLD, CYAN, RESET);
    println!("  {}  quit/exit  - End the chat session{}", DIM, RESET);
    println!("  {}  help       - Show this help message{}", DIM, RESET);
    println!("  {}  list       - Show current schedules{}", DIM, RESET);
    println!();
}

fn execute_schedule_add(params: &ScheduleParams) -> Result<()> {
    let mut initial_run: Option<chrono::DateTime<Utc>> = None;

    if let Some(ref time) = params.time {
        match schedule_utils::parse_date_time(params.date.as_deref(), Some(time)) {
            Ok(dt) => initial_run = Some(dt),
            Err(e) => {
                print_error(&format!("Failed to parse time: {:?}", e));
                return Ok(());
            }
        }
    }

    if initial_run.is_none() {
        if let Some(ref recurrence) = params.recurrence {
            if let Some(d) = schedule_utils::parse_duration_token(recurrence) {
                initial_run = Some(Utc::now() + d);
            }
        }
    }

    let run_at = initial_run.unwrap_or_else(|| Utc::now());
    let recur = params.recurrence.clone();

    let mut file = match config::load_schedules() {
        Ok(f) => f,
        Err(e) => {
            print_error(&format!("Failed to load schedules: {:?}", e));
            return Ok(());
        }
    };

    let entry = config::ScheduleEntry {
        id: config::generate_id(),
        title: params.title.clone(),
        cmd: params.cmd.clone(),
        run_at,
        recurrence: recur,
        created_at: Utc::now(),
        note: None,
    };

    file.schedules.push(entry);
    if let Err(e) = config::save_schedules(&file) {
        print_error(&format!("Failed to save schedule: {:?}", e));
        return Ok(());
    }

    let run_at_hkt = run_at.with_timezone(&chrono_tz::Asia::Hong_Kong).format("%Y-%m-%d %H:%M:%S %Z");
    print_success(&format!("Schedule added! Run at: {}", run_at_hkt));
    Ok(())
}

fn process_message(model: &str, session: &str, msg: &str) -> Result<Option<ScheduleParams>> {
    let response_raw = call_nine_poe(model, session, msg)?;
    let response = parse_chat_response(&response_raw)?;

    if response.needs_clarification {
        if let Some(question) = &response.clarification_question {
            print_ai_msg(question);
        }
        return Ok(None);
    }

    match response.params {
        Some(p) => Ok(Some(p)),
        None => {
            print_ai_msg("I couldn't understand the scheduling request. Please try again with more details.");
            Ok(None)
        }
    }
}

pub fn run_chat(title: &str, msg: &str, interactive: bool, auto_yes: bool) -> Result<()> {
    let chat_config = config::load_chat_config()?;
    let model = &chat_config.model;

    if interactive {
        run_interactive(title, model)
    } else if msg.is_empty() {
        eprintln!("Error: --msg is required when not using --interactive mode");
        eprintln!("Usage: nine-cron chat --title \"{}\" --msg \"your message\"", title);
        eprintln!("   or: nine-cron chat --title \"{}\" --interactive", title);
        std::process::exit(1);
    } else {
        run_single(title, msg, model, auto_yes)
    }
}

fn run_single(title: &str, msg: &str, model: &str, auto_yes: bool) -> Result<()> {
    print_header(title);
    print_user_msg(msg);

    match process_message(model, title, msg)? {
        Some(params) => {
            let cmd = build_schedule_command(&params);
            print_command(&cmd);

            if auto_yes {
                execute_schedule_add(&params)?;
            } else {
                println!("  {}Execute? (y/N): {}", YELLOW, RESET);
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let answer = input.trim().to_lowercase();

                match answer.as_str() {
                    "y" | "yes" => {
                        execute_schedule_add(&params)?;
                    }
                    _ => {
                        println!("  {}{}Cancelled.{}", DIM, RESET, RESET);
                    }
                }
            }
        }
        None => {
            println!("  {}Provide more details with:{}", DIM, RESET);
            println!("  {}  nine-cron chat --title \"{}\" --msg \"<your response>\"{}", DIM, title, RESET);
        }
    }

    println!();
    Ok(())
}

fn run_interactive(title: &str, model: &str) -> Result<()> {
    print_header(title);
    println!("  {}Starting interactive mode...{}", CYAN, RESET);
    println!();

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());

    loop {
        print!("  {}👤 You: {}", GREEN, RESET);
        io::stdout().flush()?;

        let mut input = String::new();
        reader.read_line(&mut input)?;
        let input = input.trim().to_string();

        if input.is_empty() {
            continue;
        }

        match input.to_lowercase().as_str() {
            "quit" | "exit" => {
                println!();
                print_success("Goodbye! 👋");
                println!();
                break;
            }
            "help" => {
                print_help();
                continue;
            }
            "list" => {
                match config::load_schedules() {
                    Ok(file) => {
                        if file.schedules.is_empty() {
                            println!("  {}No schedules found.{}", DIM, RESET);
                        } else {
                            println!();
                            println!("  {}{}Current Schedules:{}", BOLD, CYAN, RESET);
                            println!();
                            for s in file.schedules.iter().take(10) {
                                let run_at_hkt = s.run_at.with_timezone(&chrono_tz::Asia::Hong_Kong).format("%m/%d %H:%M");
                                println!("  {}{}{}  {}  {}  {}{}",
                                    BOLD, s.title, RESET,
                                    run_at_hkt,
                                    s.recurrence.as_deref().unwrap_or("once"),
                                    DIM, s.cmd);
                            }
                            println!();
                        }
                    }
                    Err(e) => {
                        print_error(&format!("Failed to load schedules: {:?}", e));
                    }
                }
                continue;
            }
            _ => {}
        }

        print_user_msg(&input);

        match process_message(model, title, &input)? {
            Some(params) => {
                let cmd = build_schedule_command(&params);
                print_command(&cmd);

                print!("  {}Execute? (y/N/modify): {}", YELLOW, RESET);
                io::stdout().flush()?;

                let mut confirm = String::new();
                reader.read_line(&mut confirm)?;
                let confirm = confirm.trim().to_lowercase();

                match confirm.as_str() {
                    "y" | "yes" => {
                        execute_schedule_add(&params)?;
                    }
                    "modify" => {
                        println!("  {}Enter modifications:{}", CYAN, RESET);
                        print!("  {}👤 You: {}", GREEN, RESET);
                        io::stdout().flush()?;
                        let mut modified = String::new();
                        reader.read_line(&mut modified)?;
                        let modified = modified.trim().to_string();

                        if !modified.is_empty() {
                            print_user_msg(&modified);
                            match process_message(model, title, &modified)? {
                                Some(new_params) => {
                                    let new_cmd = build_schedule_command(&new_params);
                                    print_command(&new_cmd);

                                    print!("  {}Execute? (y/N): {}", YELLOW, RESET);
                                    io::stdout().flush()?;

                                    let mut confirm2 = String::new();
                                    reader.read_line(&mut confirm2)?;

                                    if confirm2.trim().to_lowercase() == "y" {
                                        execute_schedule_add(&new_params)?;
                                    } else {
                                        println!("  {}{}Cancelled.{}", DIM, RESET, RESET);
                                    }
                                }
                                None => {
                                    print_ai_msg("Could not understand modifications. Please try again.");
                                }
                            }
                        }
                    }
                    _ => {
                        println!("  {}{}Cancelled.{}", DIM, RESET, RESET);
                    }
                }
            }
            None => {
                println!("  {}Provide more details or type 'help' for commands.{}", DIM, RESET);
            }
        }

        println!();
    }

    Ok(())
}
