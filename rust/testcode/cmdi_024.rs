//! CWE-78: User-controlled binary name passed directly to Command::new.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi024
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let output = Command::new(req.param("bin")) // vuln-code-snippet target-line testcodeCmdi024
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
// vuln-code-snippet end testcodeCmdi024
