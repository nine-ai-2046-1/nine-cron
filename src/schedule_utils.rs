use chrono::{Duration, NaiveDate, NaiveTime, DateTime, Utc, TimeZone};
use chrono_tz::Asia::Hong_Kong;
use anyhow;

/// Parse duration tokens like +6s, 1h30m, 15m, 1d
pub fn parse_duration_token(s: &str) -> Option<Duration> {
    let mut s = s.trim();
    if s.starts_with('+') || s.starts_with('-') {
        s = &s[1..];
    }
    // accept sequences like 1h30m or 90s
    let mut total = Duration::zero();
    let mut num = String::new();
    for ch in s.chars() {
        if ch.is_ascii_digit() {
            num.push(ch);
            continue;
        }
        if num.is_empty() { return None; }
        let val: i64 = num.parse().ok()?;
        match ch {
            's' => total = total + Duration::seconds(val),
            'm' => total = total + Duration::minutes(val),
            'h' => total = total + Duration::hours(val),
            'd' => total = total + Duration::days(val),
            _ => return None,
        }
        num.clear();
    }
    if !num.is_empty() {
        // trailing number without unit -> treat as seconds
        if let Ok(v) = num.parse::<i64>() { total = total + Duration::seconds(v); }
    }
    Some(total)
}

/// Parse date and time into UTC DateTime. If only time provided, choose next occurrence of that time in local timezone.
pub fn parse_date_time(date: Option<&str>, time: Option<&str>) -> anyhow::Result<DateTime<Utc>> {
    // Interpret provided date/time in Hong Kong Time (HKT) and convert to UTC for storage.
    let now_hkt = Utc::now().with_timezone(&Hong_Kong);
    if let Some(dstr) = date {
        // parse YYYY-MM-DD
        let date = NaiveDate::parse_from_str(dstr, "%Y-%m-%d")?;
        let time = if let Some(tstr) = time {
            NaiveTime::parse_from_str(tstr, "%H:%M")?
        } else {
            NaiveTime::from_hms_opt(0,0,0).ok_or_else(|| anyhow::anyhow!("invalid time"))?
        };
        let local_dt = Hong_Kong.from_local_datetime(&date.and_time(time)).single().ok_or_else(|| anyhow::anyhow!("ambiguous local datetime"))?;
        Ok(local_dt.with_timezone(&Utc))
    } else if let Some(tstr) = time {
        let t = NaiveTime::parse_from_str(tstr, "%H:%M")?;
        let today = now_hkt.date_naive();
        let candidate = Hong_Kong.from_local_datetime(&today.and_time(t)).single().ok_or_else(|| anyhow::anyhow!("ambiguous local datetime"))?;
        if candidate > now_hkt {
            Ok(candidate.with_timezone(&Utc))
        } else {
            let tomorrow = today.succ_opt().ok_or_else(|| anyhow::anyhow!("date overflow"))?;
            let cand2 = Hong_Kong.from_local_datetime(&tomorrow.and_time(t)).single().ok_or_else(|| anyhow::anyhow!("ambiguous local datetime"))?;
            Ok(cand2.with_timezone(&Utc))
        }
    } else {
        Err(anyhow::anyhow!("no date or time provided"))
    }
}

/// Compute next run time given current run_at and recurrence string (e.g. "1d" or "+1h")
pub fn compute_next_run_at(current: DateTime<Utc>, recurrence: &str) -> Option<DateTime<Utc>> {
    parse_duration_token(recurrence).map(|d| current + d)
}

/// Sanitize title for storing: keep alphanumeric, space, dash, underscore; collapse spaces; trim; limit length.
pub fn sanitize_title(input: &str) -> String {
    let mut out = String::new();
    let mut last_was_space = false;
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            out.push(ch);
            last_was_space = false;
        } else if ch.is_whitespace() {
            if !last_was_space {
                out.push(' ');
                last_was_space = true;
            }
        } else {
            // skip other special characters
        }
    }
    let s = out.trim().to_string();
    if s.len() > 200 { s[..200].to_string() } else { s }
}
