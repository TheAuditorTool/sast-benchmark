//! Command Injection True Positive — CWE-78
//! User-controlled env var injected into Command via .env() call.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi003Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("config");

    // VULNERABLE: User-controlled value passed as environment variable
    let output = Command::new("app")
        .env("CONFIG", &user_input) // vuln-code-snippet vuln-line testcodeCmdi003Vulnerable
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
// vuln-code-snippet end testcodeCmdi003Vulnerable
