//! CWE-78: Piped shell command built from user input via format!.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_file = req.param("file");

    let shell_cmd = format!("cat {} | grep error", user_file); // vuln-code-snippet target-line testcodeCmdi004

    let output = Command::new("sh")
        .arg("-c")
        .arg(&shell_cmd)
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
// vuln-code-snippet end testcodeCmdi004
