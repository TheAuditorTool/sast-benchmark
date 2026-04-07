//! CWE-78: Safe — user input assigned then immediately overwritten with a hardcoded value.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut cmd = req.param("cmd");
    cmd = "ls".to_string();

    let output = Command::new(&cmd) // vuln-code-snippet target-line testcodeCmdi030
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
// vuln-code-snippet end testcodeCmdi030
