//! CWE-78: Args filtered to only alphanumeric, hyphen, and dot characters.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi006
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw_arg = req.param("arg");

    let filtered: String = raw_arg.chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '.') // vuln-code-snippet target-line testcodeCmdi006
        .collect();

    let output = Command::new("ls")
        .arg(&filtered)
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
// vuln-code-snippet end testcodeCmdi006
