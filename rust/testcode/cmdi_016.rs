//! CWE-78: User-controlled grep pattern injected as a direct argument.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let output = Command::new("grep")
        .arg(req.param("pattern")) // vuln-code-snippet target-line testcodeCmdi016
        .arg("/etc/logs")
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
// vuln-code-snippet end testcodeCmdi016
