use std::process::{Command, Stdio};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let repo = req.param("repo");
    let branch = req.cookie("branch");
    let cmd = format!("git -C /repos/{} log --oneline {}", repo, branch);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();
    match output {
        Ok(o) => super::shared::BenchmarkResponse::ok(&String::from_utf8_lossy(&o.stdout).to_string()),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
