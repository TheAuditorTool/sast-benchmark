use std::process::{Command, Stdio};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("dir");

    if !input.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return super::shared::BenchmarkResponse::bad_request("Invalid directory name");
    }

    let dir = input;

    let output = Command::new("ls")
        .arg(&dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(o) => super::shared::BenchmarkResponse::ok(
            &String::from_utf8_lossy(&o.stdout).to_string()
        ),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
