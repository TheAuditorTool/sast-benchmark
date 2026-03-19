//! Command Injection True Positive — CWE-78
//! User-controlled filename passed to shell command via format!().
//! Attacker can inject shell metacharacters: filename="; rm -rf /"

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi003Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");

    let shell_cmd = format!("wc -l {}", filename); // vuln-code-snippet vuln-line testcodeCmdi003Vulnerable
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
// vuln-code-snippet end testcodeCmdi003Vulnerable
