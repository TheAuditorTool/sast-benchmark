//! CWE-78: Unsanitized user input passed to sh -c via Command::arg.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let cmd = req.param("cmd");

    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd) // vuln-code-snippet target-line testcodeCmdi015
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
// vuln-code-snippet end testcodeCmdi015
