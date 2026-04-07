//! CWE-78: Taint flows through format! into sh -c argument.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let cmd_str = format!("ls {}", req.param("dir"));

    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd_str) // vuln-code-snippet target-line testcodeCmdi020
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
// vuln-code-snippet end testcodeCmdi020
