//! Command Injection True Negative — CWE-78
//! Shell escape via single-quote wrapping to prevent injection.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi012Safe
fn shell_escape(input: &str) -> String {
    format!("'{}'", input.replace('\'', "'\\''")) // vuln-code-snippet safe-line testcodeCmdi012Safe
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_arg = req.param("arg");

    // SAFE: User input escaped via single-quote wrapping
    let escaped = shell_escape(&user_arg);
    let shell_cmd = format!("echo {}", escaped);

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
// vuln-code-snippet end testcodeCmdi012Safe
