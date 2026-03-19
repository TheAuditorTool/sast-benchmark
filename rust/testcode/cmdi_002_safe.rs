//! Command Injection True Negative — CWE-78
//! Command name from allowlist only. User input selects from predefined commands.
//! Isomorphic to cmdi_001_vulnerable — same parameter shape, safe dispatch.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi002Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let cmd_type = req.param("cmd");
    let arg = req.param("arg");

    // SAFE: Command name from allowlist, not from user input
    let command = match cmd_type.as_str() { // vuln-code-snippet safe-line testcodeCmdi002Safe
        "ls" => "ls",
        "date" => "date",
        "whoami" => "whoami",
        _ => return super::shared::BenchmarkResponse::bad_request("Unknown command"),
    };

    let output = Command::new(command)
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
// vuln-code-snippet end testcodeCmdi002Safe
