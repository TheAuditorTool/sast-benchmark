//! CWE-78: Safe — hardcoded command; user input selects only between two known flags.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi032
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let output = Command::new("ls") // vuln-code-snippet target-line testcodeCmdi032
        .arg(if req.param("fmt") == "long" { "-la" } else { "-a" })
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
// vuln-code-snippet end testcodeCmdi032
