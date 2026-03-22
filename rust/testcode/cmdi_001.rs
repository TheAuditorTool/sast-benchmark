//! CWE-78: User-controlled command name passed directly to Command::new().

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let cmd = req.param("cmd");
    let arg = req.param("arg");

    let output = Command::new(&cmd) // vuln-code-snippet target-line testcodeCmdi001
        .arg(&arg)
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
// vuln-code-snippet end testcodeCmdi001
