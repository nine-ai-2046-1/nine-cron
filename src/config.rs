use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use anyhow;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScheduleEntry {
    pub id: String,
    // store the full command as a single string (quoted by user when necessary)
    pub cmd: String,
    pub title: String,
    pub run_at: DateTime<Utc>,
    pub recurrence: Option<String>,
    pub created_at: DateTime<Utc>,
    pub note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScheduleFile { pub schedules: Vec<ScheduleEntry> }

fn config_path() -> anyhow::Result<PathBuf> {
    let proj = ProjectDirs::from("com", "example", "nine-cron")
        .ok_or_else(|| anyhow::anyhow!("cannot determine config dir"))?;
    let dir = proj.config_dir();
    fs::create_dir_all(dir)?;
    Ok(dir.join("schedules.toml"))
}

pub fn load_schedules() -> anyhow::Result<ScheduleFile> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(ScheduleFile::default());
    }
    let s = fs::read_to_string(&path)?;
    // Try parsing into the current structure first.
    match toml::from_str::<ScheduleFile>(&s) {
        Ok(f) => Ok(f),
        Err(e) => {
            // parsing as current struct failed -- attempt to read raw TOML and be conservative
            eprintln!("warning: schedules file uses older format or is missing fields: {:?}", e);
            // Fallback: parse as TOML value and attempt a resilient migration
            let v: toml::Value = toml::from_str(&s).map_err(|e| anyhow::anyhow!(e))?;
            let tables = v.get("schedules").and_then(|t| t.as_array()).ok_or_else(|| anyhow::anyhow!("invalid legacy schedules format"))?;
            let mut new = ScheduleFile::default();
            let mut skipped: Vec<String> = Vec::new();
            for item in tables.iter() {
                if let Some(tbl) = item.as_table() {
                    // require a title field in the on-disk data; do not auto-add schedules missing title
                    if tbl.get("title").and_then(|x| x.as_str()).is_none() {
                        let id = tbl.get("id").and_then(|x| x.as_str()).map(|s| s.to_string()).unwrap_or_else(|| "<unknown>".to_string());
                        skipped.push(id);
                        continue;
                    }
                    // id
                    let id = tbl.get("id").and_then(|x| x.as_str()).map(|s| s.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string());
                    // cmd may be string or array
                    let cmd = match tbl.get("cmd") {
                        Some(vv) => {
                            if let Some(s) = vv.as_str() { s.to_string() }
                            else if let Some(arr) = vv.as_array() { arr.iter().filter_map(|e| e.as_str()).collect::<Vec<_>>().join(" ") }
                            else { String::new() }
                        }
                        None => String::new(),
                    };
                    // run_at
                    let run_at = tbl.get("run_at").and_then(|x| x.as_str()).and_then(|s| DateTime::parse_from_rfc3339(s).ok()).map(|dt| dt.with_timezone(&Utc)).unwrap_or_else(|| Utc::now());
                    // recurrence
                    let recurrence = tbl.get("recurrence").and_then(|x| x.as_str()).map(|s| s.to_string());
                    // created_at
                    let created_at = tbl.get("created_at").and_then(|x| x.as_str()).and_then(|s| DateTime::parse_from_rfc3339(s).ok()).map(|dt| dt.with_timezone(&Utc)).unwrap_or_else(|| Utc::now());
                    // note
                    let note = tbl.get("note").and_then(|x| x.as_str()).map(|s| s.to_string());

                    let title = tbl.get("title").and_then(|x| x.as_str()).map(|s| s.to_string()).unwrap_or_else(|| crate::schedule_utils::sanitize_title(&cmd));

                    let entry = ScheduleEntry { id, cmd, title, run_at, recurrence, created_at, note };
                    new.schedules.push(entry);
                }
            }
            if !skipped.is_empty() {
                eprintln!("warning: {} schedule(s) in {} were not migrated because they are missing a title. IDs: {:?}", skipped.len(), path.display(), skipped);
                eprintln!("Please re-add those schedules using: nine-cron schedule add -T \"TITLE\" <args> or edit the schedules.toml manually.");
            }
            Ok(new)
        }
    }
}

pub fn save_schedules(file: &ScheduleFile) -> anyhow::Result<()> {
    let path = config_path()?;
    let tom = toml::to_string_pretty(file)?;
    fs::write(path, tom)?;
    Ok(())
}

pub fn runs_dir() -> anyhow::Result<std::path::PathBuf> {
    let proj = ProjectDirs::from("com", "example", "nine-cron")
        .ok_or_else(|| anyhow::anyhow!("cannot determine config dir"))?;
    let dir = proj.data_dir().join("runs");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn prune_runs(keep: usize) -> anyhow::Result<()> {
    let dir = runs_dir()?;
    let mut entries: Vec<_> = fs::read_dir(&dir)?.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());
    if entries.len() <= keep { return Ok(()); }
    let to_remove = entries.len() - keep;
    for e in entries.into_iter().take(to_remove) { let _ = fs::remove_file(e.path()); }
    Ok(())
}
