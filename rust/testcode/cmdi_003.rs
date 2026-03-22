//! CWE-78: User-controlled filename passed to shell command via format!().

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");

    let shell_cmd = format!("wc -l {}", filename); // vuln-code-snippet target-line testcodeCmdi003
    let output = Command::new("sh")
        .args(["-c", &shell_cmd])
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
// vuln-code-snippet end testcodeCmdi003
