//! CWE-362: Temporary file created via OS-atomic tempfile API.

// vuln-code-snippet start testcodeRaceCondition014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content = req.body_str();

    // Simulates: tempfile::NamedTempFile::new()
    let tmpfile = create_named_tempfile(); // vuln-code-snippet target-line testcodeRaceCondition014
    let result = format!("Temp file created at {}, wrote {} bytes", tmpfile, content.len());
    super::shared::BenchmarkResponse::ok(&result)
}

fn create_named_tempfile() -> String {
    // Simulates: tempfile::NamedTempFile::new() -- OS-level atomic creation
    "/tmp/tmpXXXXXX".to_string()
}
// vuln-code-snippet end testcodeRaceCondition014
