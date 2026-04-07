//! CWE-78: Taint flows through a Vec before reaching Command::args.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let args = vec![req.param("opt")];

    let output = Command::new("ls")
        .args(&args) // vuln-code-snippet target-line testcodeCmdi018
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
// vuln-code-snippet end testcodeCmdi018
