// using crate::config (src/config.rs)
use std::process::{Command, Stdio};
use std::io::{self, BufRead};
use std::thread;
use std::sync::mpsc::{self, Sender};
use chrono::{DateTime, Utc};
use std::io::Write;

/// Result of a run: exit code and captured stdout/stderr.
#[derive(Clone)]
pub struct RunResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub run_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

pub struct StreamLine {
    pub stream: String,
    pub text: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug)]
pub enum RunError {
    Io(String),
}

impl From<io::Error> for RunError {
    fn from(e: io::Error) -> Self { RunError::Io(e.to_string()) }
}

pub trait Runner {
    fn run(&self, cmd: &str, args: &[String], tx: Option<Sender<StreamLine>>, run_id: &str) -> Result<RunResult, RunError>;
}

pub struct SystemRunner;

impl Runner for SystemRunner {
    fn run(&self, cmd: &str, args: &[String], tx: Option<Sender<StreamLine>>, run_id: &str) -> Result<RunResult, RunError> {
        let start_time = Utc::now();
        let run_id = run_id.to_string();

        let mut child = Command::new(cmd)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().ok_or_else(|| RunError::Io("missing stdout".into()))?;
        let stderr = child.stderr.take().ok_or_else(|| RunError::Io("missing stderr".into()))?;

        let tx_stdout = tx.clone();
        thread::spawn(move || {
            let reader = io::BufReader::new(stdout);
            for line in reader.lines() {
                match line { Ok(l) => { if let Some(ref s) = tx_stdout { let _ = s.send(StreamLine { stream: "stdout".into(), text: l, timestamp: Utc::now()}); } }, Err(_) => break }
            }
        });

        let tx_stderr = tx.clone();
        thread::spawn(move || {
            let reader = io::BufReader::new(stderr);
            for line in reader.lines() {
                match line { Ok(l) => { if let Some(ref s) = tx_stderr { let _ = s.send(StreamLine { stream: "stderr".into(), text: l, timestamp: Utc::now()}); } }, Err(_) => break }
            }
        });

        let status = child.wait()?;
        let end_time = Utc::now();
        let exit_code = status.code().unwrap_or_else(|| if status.success() { 0 } else { 1 });
        let runs = crate::config::runs_dir().ok();
        if let Some(dir) = runs {
            let path = dir.join(format!("{}.log", run_id)); // PathBuf inferred here
            if let Ok(mut f) = std::fs::File::create(&path) {
                let _ = writeln!(f, "{{\"run_id\":\"{}\",\"start_time\":\"{}\"}}", run_id, start_time.to_rfc3339());
            }
        }
        Ok(RunResult { exit_code, stdout: String::new(), stderr: String::new(), run_id: run_id.clone(), start_time, end_time })
    }
}

pub struct MockRunner { pub result: Result<RunResult, RunError> }

impl Runner for MockRunner {
    fn run(&self, _cmd: &str, _args: &[String], tx: Option<Sender<StreamLine>>, _run_id: &str) -> Result<RunResult, RunError> {
        match &self.result {
            Ok(r) => {
                if let Some(sender) = tx {
                    let stdout = r.stdout.clone();
                    let stderr = r.stderr.clone();
                    let _ = std::thread::spawn(move || {
                        for line in stdout.lines() { let _ = sender.send(StreamLine { stream: "stdout".into(), text: line.to_string(), timestamp: Utc::now() }); }
                        for line in stderr.lines() { let _ = sender.send(StreamLine { stream: "stderr".into(), text: line.to_string(), timestamp: Utc::now() }); }
                    });
                }
                Ok(RunResult { exit_code: r.exit_code, stdout: r.stdout.clone(), stderr: r.stderr.clone(), run_id: r.run_id.clone(), start_time: r.start_time, end_time: r.end_time })
            }
            Err(e) => Err(RunError::Io(format!("mock error: {:?}", e)))
        }
    }
}

pub fn run_with_runner<R: Runner>(runner: &R, args: &[String]) -> i32 {
    if args.is_empty() { eprintln!("no command provided"); return 2; }
    let mut parts = args.to_vec();
    if parts[0] == "run" { parts.remove(0); }
    if !parts.is_empty() && parts[0] == "--" { parts.remove(0); }
    if parts.is_empty() { eprintln!("no command after run"); return 2; }
    // If user provided a single quoted command string (e.g. "date > test.log")
    // treat it as a shell command and execute via `sh -c` (or `cmd /C` on Windows).
    let (cmd, cmd_args): (String, Vec<String>) = if parts.len() == 1 {
        let single = parts[0].clone();
        // detect whitespace or shell special characters that indicate need for shell parsing
        let need_shell = single.contains(' ') || single.contains('|') || single.contains('>') || single.contains('<') || single.contains('&') || single.contains('$') || single.contains('*') || single.contains('(') || single.contains(')');
        if need_shell {
            if cfg!(windows) {
                ("cmd".to_string(), vec!['/'.to_string() + "C", single])
            } else {
                ("sh".to_string(), vec!["-c".to_string(), single])
            }
        } else {
            // no special chars: treat as single executable with no args
            (single.clone(), Vec::new())
        }
    } else {
        (parts[0].clone(), parts.iter().skip(1).cloned().collect())
    };
    let (tx, rx) = mpsc::channel();
    let tx_opt = Some(tx.clone());
    let run_id = crate::config::generate_id();
    match runner.run(&cmd, &cmd_args, tx_opt, &run_id) {
        Ok(res) => {
            let run_id = res.run_id.clone();
            let handle = std::thread::spawn(move || {
                // try open run log file for append
                let run_path = crate::config::runs_dir().ok().map(|d| d.join(format!("{}.log", run_id)));
                let mut file = run_path.as_ref().and_then(|p| std::fs::OpenOptions::new().create(true).append(true).open(p).ok());
                for sl in rx {
                // present stream timestamps in HKT for readability
                let ts_hkt = sl.timestamp.with_timezone(&chrono_tz::Asia::Hong_Kong).to_rfc3339();
                let obj = serde_json::json!({"run_id": run_id, "timestamp": ts_hkt, "stream": sl.stream, "text": sl.text});
                    let line = obj.to_string();
                    println!("{}", line);
                    if let Some(f) = file.as_mut() {
                        let _ = writeln!(f, "{}", line);
                    }
                }
            });
            drop(tx);
            let _ = handle.join();
            let start_hkt = res.start_time.with_timezone(&chrono_tz::Asia::Hong_Kong).to_rfc3339();
            let end_hkt = res.end_time.with_timezone(&chrono_tz::Asia::Hong_Kong).to_rfc3339();
            let meta = serde_json::json!({"run_id": res.run_id, "start_time": start_hkt, "end_time": end_hkt, "exit_code": res.exit_code});
            println!("{}", meta.to_string());
            if let Ok(dir) = crate::config::runs_dir() {
                let path = dir.join(format!("{}.log", res.run_id));
                if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
                    let _ = writeln!(f, "{}", meta.to_string());
                }
            }
            res.exit_code
        }
        Err(e) => { let obj = serde_json::json!({"error": format!("{:?}", e)}); println!("{}", obj.to_string()); 1 }
    }
}

#[cfg(test)]
mod tests { use super::*; 
    #[test]
    fn test_run_with_mock_success() {
        let rr = RunResult { exit_code: 0, stdout: "ok\n".into(), stderr: "".into(), run_id: "r1".into(), start_time: Utc::now(), end_time: Utc::now() };
        let mock = MockRunner { result: Ok(rr) };
        let args = vec!["run".to_string(), "echo".to_string(), "hello".to_string()];
        let code = run_with_runner(&mock, &args);
        assert_eq!(code, 0);
    }

    #[test]
    fn test_single_quoted_shell_command_uses_sh() {
        // runner that captures what cmd and args were called with
        struct CaptureRunner { pub cmd: std::sync::Mutex<Option<(String, Vec<String>)>>, pub result: Result<RunResult, RunError> }
        impl Runner for CaptureRunner {
            fn run(&self, cmd: &str, args: &[String], _tx: Option<Sender<StreamLine>>, _run_id: &str) -> Result<RunResult, RunError> {
                let mut lock = self.cmd.lock().unwrap();
                *lock = Some((cmd.to_string(), args.to_vec()));
                match &self.result { Ok(r) => Ok(r.clone()), Err(e) => Err(RunError::Io(format!("mock: {:?}", e))) }
            }
        }
        let rr = RunResult { exit_code: 0, stdout: "ok\n".into(), stderr: "".into(), run_id: "r1".into(), start_time: Utc::now(), end_time: Utc::now() };
        let runner = CaptureRunner { cmd: std::sync::Mutex::new(None), result: Ok(rr) };
        let args = vec!["run".to_string(), "date > test.log".to_string()];
        let _ = run_with_runner(&runner, &args);
        let called = runner.cmd.lock().unwrap().clone().expect("should have been set");
        if cfg!(windows) {
            assert_eq!(called.0, "cmd");
            assert_eq!(called.1, vec!["/C".to_string(), "date > test.log".to_string()]);
        } else {
            assert_eq!(called.0, "sh");
            assert_eq!(called.1, vec!["-c".to_string(), "date > test.log".to_string()]);
        }
    }
}
