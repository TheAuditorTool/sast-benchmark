//! CWE-78: User input injected via environment variable that the shell script expands.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let output = Command::new("bash")
        .env("DIR", req.param("dir")) // vuln-code-snippet target-line testcodeCmdi023
        .arg("-c")
        .arg("ls $DIR")
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
// vuln-code-snippet end testcodeCmdi023
