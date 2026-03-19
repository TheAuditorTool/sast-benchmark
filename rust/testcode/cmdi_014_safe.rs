//! Command Injection True Negative — CWE-78
//! Max argument length enforced (256 chars) to limit injection surface.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi014Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_arg = req.param("arg");

    // SAFE: Enforce max argument length
    if user_arg.len() > 256 { // vuln-code-snippet safe-line testcodeCmdi014Safe
        return super::shared::BenchmarkResponse::bad_request("Argument too long");
    }

    let output = Command::new("echo")
        .arg(&user_arg)
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
// vuln-code-snippet end testcodeCmdi014Safe
