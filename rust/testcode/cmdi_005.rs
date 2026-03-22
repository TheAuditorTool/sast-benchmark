//! CWE-78: Env var from hardcoded config, not user input.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi005
pub fn handle(_req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let config_value = "production";

    let output = Command::new("app")
        .env("CONFIG", config_value) // vuln-code-snippet target-line testcodeCmdi005
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
// vuln-code-snippet end testcodeCmdi005
