use std::collections::HashMap;
use std::process::{Command, Stdio};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut m: HashMap<&str, String> = HashMap::new();
    m.insert("user_cmd", req.param("cmd"));
    m.insert("safe_cmd", "date".to_string());

    let exe = m.get("safe_cmd").unwrap();

    let output = Command::new(exe)
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
