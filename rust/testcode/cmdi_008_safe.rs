//! Command Injection True Negative — CWE-78
//! Arguments passed as array to .args(), not interpolated into shell string.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi008Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let pattern = req.param("pattern");
    let filename = req.param("file");

    // SAFE: Arguments passed as separate array elements, not shell string
    let output = Command::new("grep")
        .args(&[&pattern, &filename]) // vuln-code-snippet safe-line testcodeCmdi008Safe
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
// vuln-code-snippet end testcodeCmdi008Safe
