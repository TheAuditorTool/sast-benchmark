//! CWE-78: Incomplete sanitization — semicolons blocked but backtick injection remains possible.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi021
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("cmd");

    if input.contains(';') {
        return super::shared::BenchmarkResponse::bad_request("Semicolons not allowed");
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg(&input) // vuln-code-snippet target-line testcodeCmdi021
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
// vuln-code-snippet end testcodeCmdi021
