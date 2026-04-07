//! CWE-78: Safe — explicit allowlist maps user action to a hardcoded command name.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let cmd_name = match req.param("action").as_str() {
        "date"   => "date",
        "whoami" => "whoami",
        _        => return super::shared::BenchmarkResponse::forbidden("Command not allowed"),
    };

    let output = Command::new(cmd_name) // vuln-code-snippet target-line testcodeCmdi026
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
// vuln-code-snippet end testcodeCmdi026
