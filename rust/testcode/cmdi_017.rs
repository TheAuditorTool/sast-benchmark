//! CWE-78: User-controlled directory passed as the first argument to find.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let output = Command::new("find")
        .arg(req.param("dir")) // vuln-code-snippet target-line testcodeCmdi017
        .arg("-type")
        .arg("f")
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
// vuln-code-snippet end testcodeCmdi017
