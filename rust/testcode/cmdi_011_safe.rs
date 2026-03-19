//! Command Injection True Negative — CWE-78
//! Path argument validated as existing file before passing to command.

use std::path::Path;
use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi011Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filepath = req.param("path");
    let path = Path::new(&filepath);

    // SAFE: Validate path exists as a regular file before use
    if !path.exists() || !path.is_file() { // vuln-code-snippet safe-line testcodeCmdi011Safe
        return super::shared::BenchmarkResponse::bad_request("Invalid file path");
    }

    let output = Command::new("cat")
        .arg(&filepath)
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
// vuln-code-snippet end testcodeCmdi011Safe
