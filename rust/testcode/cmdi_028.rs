//! CWE-78: Safe — character allowlist validation rejects input before passing to echo.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi028
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("value");

    if !input.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return super::shared::BenchmarkResponse::bad_request("Invalid characters in input");
    }

    let validated = input;

    let output = Command::new("echo")
        .arg(&validated) // vuln-code-snippet target-line testcodeCmdi028
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
// vuln-code-snippet end testcodeCmdi028
