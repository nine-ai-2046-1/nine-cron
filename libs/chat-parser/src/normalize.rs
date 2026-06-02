use serde_json::Value;
use regex::Regex;
use lazy_static::lazy_static;

use crate::types::{NormalizedChatResponse, NormalizedParams};
use crate::errors::ParseError;

fn strip_backticks(s: &str) -> String {
    // remove ```json ... ``` or ``` ... ``` fences
    let re = Regex::new(r"(?s)```(?:json)?\s*(.*?)\s*```").unwrap();
    if let Some(cap) = re.captures(s) {
        cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_else(|| s.to_string())
    } else {
        s.to_string()
    }
}

fn strip_ansi(s: &str) -> String {
    lazy_static!{
        static ref ANSI: Regex = Regex::new(r"\x1B\[[0-9;]*[a-zA-Z]").unwrap();
    }
    ANSI.replace_all(s, "").to_string()
}

fn preclean(raw: &str) -> String {
    let t = raw.trim();
    let no_ansi = strip_ansi(t);
    let no_ticks = strip_backticks(&no_ansi);
    no_ticks.trim().to_string()
}

fn try_parse_value(s: &str) -> Result<Value, ParseError> {
    serde_json::from_str::<Value>(s).map_err(|e| ParseError::Serde(format!("{}", e)))
}

fn substring_attempts(raw: &str) -> Result<Value, ParseError> {
    // find each opening brace and try to parse from there by expanding the end
    let bytes = raw.as_bytes();
    let len = bytes.len();
    for i in 0..len {
        if bytes[i] == b'{' {
            // try progressively longer substrings up to a limit
            let mut j = i+1;
            while j <= len {
                if bytes[j-1] == b'}' {
                    let candidate = &raw[i..j];
                    if let Ok(v) = serde_json::from_str::<Value>(candidate) {
                        return Ok(v);
                    }
                }
                // limit length of candidate to avoid O(n^2) blowup
                if j - i > 20000 { break; }
                j += 1;
            }
        }
    }
    Err(ParseError::Substring("no valid JSON found in substrings".to_string()))
}

fn normalize_value(v: Value) -> Result<NormalizedChatResponse, ParseError> {
    if !v.is_object() {
        return Err(ParseError::Validation("top-level JSON is not an object".to_string()));
    }

    let action = v.get("action").and_then(|x| x.as_str()).map(|s| s.to_string());
    let needs_clarification = v.get("needs_clarification").and_then(|x| x.as_bool()).unwrap_or(false);
    let clarification_question = v.get("clarification_question").and_then(|x| x.as_str()).map(|s| s.to_string());
    let top_title = v.get("title").and_then(|x| x.as_str()).map(|s| s.to_string());

    let params_opt = v.get("params");
    let mut params_normalized: Option<NormalizedParams> = None;

    if let Some(params_val) = params_opt {
        if params_val.is_object() {
            let time = params_val.get("time").and_then(|x| x.as_str()).map(|s| s.to_string());
            let date = params_val.get("date").and_then(|x| x.as_str()).map(|s| s.to_string());
            let recurrence = params_val.get("recurrence").and_then(|x| x.as_str()).map(|s| s.to_string());
            let title_in_params = params_val.get("title").and_then(|x| x.as_str()).map(|s| s.to_string()).or_else(|| top_title.clone());
            let cmd = params_val.get("cmd").and_then(|x| x.as_str()).map(|s| s.to_string());

            if let Some(cmd_s) = cmd {
                params_normalized = Some(NormalizedParams { time, date, recurrence, title: title_in_params, cmd: cmd_s });
            } else {
                return Err(ParseError::Validation("params.cmd missing or not a string".to_string()));
            }
        } else {
            return Err(ParseError::Validation("params is not an object".to_string()));
        }
    } else {
        // no params key — attempt to use top-level fields
        // fallback: try to extract time/date/cmd from top-level if present
        let cmd = v.get("cmd").and_then(|x| x.as_str()).map(|s| s.to_string());
        if let Some(cmd_s) = cmd {
            let time = v.get("time").and_then(|x| x.as_str()).map(|s| s.to_string());
            let date = v.get("date").and_then(|x| x.as_str()).map(|s| s.to_string());
            let recurrence = v.get("recurrence").and_then(|x| x.as_str()).map(|s| s.to_string());
            let title = top_title.clone();
            params_normalized = Some(NormalizedParams { time, date, recurrence, title, cmd: cmd_s });
        }
    }

    Ok(NormalizedChatResponse {
        action,
        needs_clarification,
        clarification_question,
        title: top_title,
        params: params_normalized,
    })
}

pub fn parse_and_normalize(raw: &str) -> Result<NormalizedChatResponse, ParseError> {
    let cleaned = preclean(raw);
    // 1. try parse whole
    match try_parse_value(&cleaned) {
        Ok(v) => return normalize_value(v),
        Err(_) => {
            // fallthrough to substring attempts
        }
    }

    // 2. substring attempts
    let v = substring_attempts(&cleaned)?;
    normalize_value(v)
}
