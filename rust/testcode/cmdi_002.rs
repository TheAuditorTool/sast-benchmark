//! CWE-78: Command name from allowlist. User input selects from predefined commands.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let cmd_type = req.param("cmd");
    let arg = req.param("arg");

    let command = match cmd_type.as_str() { // vuln-code-snippet target-line testcodeCmdi002
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
// vuln-code-snippet end testcodeCmdi002
