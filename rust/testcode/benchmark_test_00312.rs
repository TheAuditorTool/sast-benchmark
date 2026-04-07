use std::process::{Command, Stdio};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let tool = req.param("tool");

    if tool.contains('/') || tool.contains('\\') {
        return super::shared::BenchmarkResponse::bad_request("Absolute paths not allowed");
    }

    let output = Command::new(&tool)
        .env("PATH", "/usr/bin")
        .env_clear()
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
