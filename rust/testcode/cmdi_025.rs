//! CWE-78: Taint flows through a Vec slice into a positional Command arg.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let msg = req.param("msg");
    let parts: Vec<&str> = vec!["echo", msg.as_str()];

    let output = Command::new(parts[0])
        .arg(parts[1]) // vuln-code-snippet target-line testcodeCmdi025
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
// vuln-code-snippet end testcodeCmdi025
