//! CWE-78: Safe — helper function ignores user input and always returns a hardcoded command.

use std::process::{Command, Stdio};

fn safe_cmd(_user_input: &str) -> &'static str {
    "date"
}

// vuln-code-snippet start testcodeCmdi031
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let output = Command::new(safe_cmd(&req.param("cmd"))) // vuln-code-snippet target-line testcodeCmdi031
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
// vuln-code-snippet end testcodeCmdi031
