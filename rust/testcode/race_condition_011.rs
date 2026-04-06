//! CWE-362: File created atomically with create_new flag, failing if already exists.

// vuln-code-snippet start testcodeRaceCondition011
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filepath = req.param("path");
    let content = req.body_str();

    match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true) // vuln-code-snippet target-line testcodeRaceCondition011
        .open(&filepath)
    {
        Ok(mut f) => {
            use std::io::Write;
            let _ = f.write_all(content.as_bytes());
            super::shared::BenchmarkResponse::ok("Created")
        }
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodeRaceCondition011
