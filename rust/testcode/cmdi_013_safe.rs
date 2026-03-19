//! Command Injection True Negative — CWE-78
//! Restricted PATH env to only /usr/bin, limiting executable search.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi013Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let tool = req.param("tool");

    // SAFE: Reject absolute/relative paths to prevent bypassing PATH restriction
    if tool.contains('/') || tool.contains('\\') { // vuln-code-snippet safe-line testcodeCmdi013Safe
        return super::shared::BenchmarkResponse::bad_request("Absolute paths not allowed");
    }

    // PATH restricted to only /usr/bin, preventing arbitrary binary execution
    let output = Command::new(&tool)
        .env("PATH", "/usr/bin")
        .env_clear()
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
// vuln-code-snippet end testcodeCmdi013Safe
