//! CWE-78: Length-only check does not prevent command injection through sh -c.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("cmd");

    if input.len() > 50 {
        return super::shared::BenchmarkResponse::bad_request("Input too long");
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg(&input) // vuln-code-snippet target-line testcodeCmdi022
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
// vuln-code-snippet end testcodeCmdi022
