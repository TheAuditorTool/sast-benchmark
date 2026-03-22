//! CWE-78: Fully static command with no user input at all.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi007
pub fn handle(_req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let output = Command::new("date") // vuln-code-snippet target-line testcodeCmdi007
        .arg("+%Y-%m-%d")
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
// vuln-code-snippet end testcodeCmdi007
