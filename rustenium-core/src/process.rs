use std::process::Stdio;

use regex::Regex;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::time::{Duration, timeout};

#[derive(Debug)]
pub struct Process {
    child: Option<Child>,
}

impl Process {
    fn from_command(exe: &str, mut cmd: Command) -> Process {
        let mut child = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .expect("Failed to start process");

        if let Some(stdout) = child.stdout.take() {
            let exe_name = exe.to_string();
            tokio::spawn(async move {
                let mut reader = BufReader::new(stdout);
                let mut line = String::new();
                loop {
                    line.clear();
                    match reader.read_line(&mut line).await {
                        Ok(0) => break,
                        Ok(_) => { tracing::debug!("[{} stdout] {}", exe_name, line.trim()); }
                        Err(e) => { tracing::error!("[{} stdout] Error reading: {}", exe_name, e); break; }
                    }
                }
            });
        }

        if let Some(stderr) = child.stderr.take() {
            let exe_name = exe.to_string();
            tokio::spawn(async move {
                let mut reader = BufReader::new(stderr);
                let mut line = String::new();
                loop {
                    line.clear();
                    match reader.read_line(&mut line).await {
                        Ok(0) => break,
                        Ok(_) => { tracing::debug!("[{} stderr] {}", exe_name, line.trim()); }
                        Err(e) => { tracing::error!("[{} stderr] Error reading: {}", exe_name, e); break; }
                    }
                }
            });
        }

        Self { child: Some(child) }
    }

    pub fn create<S, I>(exe_path: S, args: I) -> Process
    where
        S: AsRef<str>,
        I: IntoIterator<Item = String>,
    {
        let exe = exe_path.as_ref();
        let mut cmd = Command::new(exe);
        let args = args.into_iter().collect::<Vec<_>>();
        tracing::info!("Starting process: '{}', args: {:?}", exe, args);
        cmd.args(args);
        Self::from_command(exe, cmd)
    }

    pub fn create_with_env<S, I>(exe_path: S, args: I, env: impl IntoIterator<Item = (String, String)>) -> Process
    where
        S: AsRef<str>,
        I: IntoIterator<Item = String>,
    {
        let exe = exe_path.as_ref();
        let mut cmd = Command::new(exe);
        cmd.args(args.into_iter().collect::<Vec<_>>()).envs(env);
        Self::from_command(exe, cmd)
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

impl Process {
    pub fn kill(&mut self) -> Result<(), crate::error::ProcessKillError> {
        if let Some(mut child) = self.child.take() {
            if let Some(pid) = child.id() {
                let pid_str = pid.to_string();
                tracing::debug!("[Process]: Killing process, PID: {}", pid_str);

                #[cfg(unix)]
                {
                    match std::process::Command::new("pkill")
                        .args(["-9", "-P", &pid_str])
                        .output()
                    {
                        Ok(output) => {
                            tracing::debug!(
                                "[Process]: pkill stdout: {}",
                                String::from_utf8_lossy(&output.stdout)
                            );
                            tracing::debug!(
                                "[Process]: pkill stderr: {}",
                                String::from_utf8_lossy(&output.stderr)
                            );
                        }
                        Err(e) => {
                            tracing::error!("[Process]: Failed to execute pkill: {}", e);
                        }
                    }
                }

                #[cfg(windows)]
                {
                    match std::process::Command::new("taskkill")
                        .args(["/F", "/T", "/PID", &pid_str])
                        .output()
                    {
                        Ok(output) => {
                            tracing::debug!(
                                "[Process]: taskkill stdout: {}",
                                String::from_utf8_lossy(&output.stdout)
                            );
                            tracing::debug!(
                                "[Process]: taskkill stderr: {}",
                                String::from_utf8_lossy(&output.stderr)
                            );
                        }
                        Err(e) => {
                            tracing::error!("[Process]: Failed to execute taskkill: {}", e);
                        }
                    }
                }
            }

            child
                .start_kill()
                .map_err(|_| crate::error::ProcessKillError)?;

            Ok(())
        } else {
            Err(crate::error::ProcessKillError)
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        let _ = self.kill();
    }
}

/// Kill the process listening on the given TCP port.
/// Used as a fallback when the stored PID is stale (e.g. Firefox launcher respawn).
pub fn kill_process_on_port(port: u16) {
    tracing::debug!("[Process]: Killing process on port {}", port);

    #[cfg(windows)]
    {
        // netstat -ano lists TCP connections with their PIDs
        let Ok(out) = std::process::Command::new("netstat").args(["-ano"]).output() else { return };
        let stdout = String::from_utf8_lossy(&out.stdout);
        let needle = format!(":{}", port);
        for line in stdout.lines() {
            if line.contains(&needle) && line.contains("LISTENING") {
                if let Some(pid_str) = line.split_whitespace().last() {
                    tracing::debug!("[Process]: Found PID {} on port {}, killing", pid_str, port);
                    let _ = std::process::Command::new("taskkill")
                        .args(["/F", "/T", "/PID", pid_str])
                        .output();
                }
            }
        }
    }

    #[cfg(unix)]
    {
        let _ = std::process::Command::new("fuser")
            .args(["-k", &format!("{}/tcp", port)])
            .output();
    }
}
