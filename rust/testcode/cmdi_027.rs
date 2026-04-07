//! CWE-78: Safe — user controls only a format flag that selects a hardcoded ls option.

use std::process::{Command, Stdio};

// vuln-code-snippet start testcodeCmdi027
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let flag = match req.param("format").as_str() {
        "long"  => "-la",
        "short" => "-a",
        _       => "-a",
    };

    let output = Command::new("ls") // vuln-code-snippet target-line testcodeCmdi027
        .arg(flag)
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
// vuln-code-snippet end testcodeCmdi027
