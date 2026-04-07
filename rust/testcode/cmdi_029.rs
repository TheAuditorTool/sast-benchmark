//! CWE-78: Safe — constant-folded condition always selects the hardcoded "date" branch.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi029
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let cmd_src = if 7 * 6 > 40 {
        "date"
    } else {
        req.param("cmd").as_str()
    };

    let output = Command::new(cmd_src) // vuln-code-snippet target-line testcodeCmdi029
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
// vuln-code-snippet end testcodeCmdi029
