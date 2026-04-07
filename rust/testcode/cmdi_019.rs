//! CWE-78: Taint flows through a struct field into Command::new.

use std::process::{Command, Stdio};

struct CmdSpec {
    prog: String,
}

// vuln-code-snippet start testcodeCmdi019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let spec = CmdSpec { prog: req.param("prog") };

    let output = Command::new(&spec.prog) // vuln-code-snippet target-line testcodeCmdi019
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
// vuln-code-snippet end testcodeCmdi019
