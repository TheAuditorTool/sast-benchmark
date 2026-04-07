use std::process::{Command, Stdio};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("value");

    if !input.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return super::shared::BenchmarkResponse::bad_request("Invalid characters in input");
    }

    let validated = input;

    let output = Command::new("echo")
        .arg(&validated)
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
