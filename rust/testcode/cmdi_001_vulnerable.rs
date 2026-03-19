//! Command Injection True Positive — CWE-78
//! User-controlled command name passed directly to Command::new().

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi001Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let cmd = req.param("cmd");
    let arg = req.param("arg");

    // VULNERABLE: User controls the command name
    let output = Command::new(&cmd) // vuln-code-snippet vuln-line testcodeCmdi001Vulnerable
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
// vuln-code-snippet end testcodeCmdi001Vulnerable
