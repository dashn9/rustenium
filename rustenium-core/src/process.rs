use std::process::Stdio;

use regex::Regex;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::time::{timeout, Duration};

pub struct Process {
    child: Option<Child>,
}

impl Process {
    pub fn create<S, I>(exe_path: S, args: I) -> Process
    where
        S: AsRef<str>,
        I: IntoIterator<Item = String>,
    {
        let exe = exe_path.as_ref();
        let args_vec: Vec<String> = args.into_iter().collect();

        // Log the full command being executed with details
        tracing::info!(
            "Starting process with executable: '{}' and arguments: {:?}",
            exe,
            args_vec
        );
        tracing::info!(
            "Full command: {} {}",
            exe,
            args_vec.join(" ")
        );

        let mut child = Command::new(exe)
            .args(args_vec)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start process");

        // Capture and log stdout
        if let Some(stdout) = child.stdout.take() {
            let exe_name = exe.to_string();
            tokio::spawn(async move {
                let mut reader = BufReader::new(stdout);
                let mut line = String::new();
                loop {
                    line.clear();
                    match reader.read_line(&mut line).await {
                        Ok(0) => break, // EOF
                        Ok(_) => {
                            tracing::debug!("[{} stdout] {}", exe_name, line.trim());
                        }
                        Err(e) => {
                            tracing::error!("[{} stdout] Error reading: {}", exe_name, e);
                            break;
                        }
                    }
                }
            });
        }

        // Capture and log stderr
        if let Some(stderr) = child.stderr.take() {
            let exe_name = exe.to_string();
            tokio::spawn(async move {
                let mut reader = BufReader::new(stderr);
                let mut line = String::new();
                loop {
                    line.clear();
                    match reader.read_line(&mut line).await {
                        Ok(0) => break, // EOF
                        Ok(_) => {
                            tracing::debug!("[{} stderr] {}", exe_name, line.trim());
                        }
                        Err(e) => {
                            tracing::error!("[{} stderr] Error reading: {}", exe_name, e);
                            break;
                        }
                    }
                }
            });
        }

        let child = Some(child);

        return Self { child };
    }

    #[deprecated]
    pub async fn wait_for_pattern(&mut self, pattern: &str, timeout_secs: Option<u64>) -> String {
        let timeout_secs = timeout_secs.unwrap_or(20);
        let regex = Regex::new(pattern).expect("Invalid regex pattern");
        let child = self.child.as_mut().unwrap();

        let stdout = child.stdout.as_mut().expect("Failed to access stdout");
        let stderr = child.stderr.as_mut().expect("Failed to access stderr");

        let mut stdout_lines = BufReader::new(stdout).lines();
        let mut stderr_lines = BufReader::new(stderr).lines();

        let check_line = |_label: &str, line: Result<Option<String>, _>| -> Option<String> {
            if let Ok(Some(line)) = line {
                if let Some(captures) = regex.captures(&line) {
                    if let Some(url) = captures.get(1) {
                        return Some(url.as_str().into());
                    }
                }
            }
            None
        };

        let timeout_duration = Duration::from_secs(timeout_secs);

        let timeout_result = timeout(timeout_duration, async {
            loop {
                tokio::select! {
                    stdout_line = stdout_lines.next_line() => {
                        if let Some(line) = check_line("stdout", stdout_line) {
                            return Some(line);
                        }
                    },
                    stderr_line = stderr_lines.next_line() => {
                        if let Some(line) = check_line("stderr", stderr_line) {
                            return  Some(line);
                        }
                    }
                }
            }
        })
        .await;

        match timeout_result {
            Ok(Some(matched)) => matched,
            Ok(None) => panic!("Found a pattern but None"),
            Err(_) => panic!("Timeout reached without finding pattern"),
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill(); // Ensure cleanup
        }
    }
}
