//! CWE-22: User-controlled file path passed directly to fs::remove_file().

// vuln-code-snippet start testcodePathtraver019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let file = req.param("file");

    match std::fs::remove_file(&file) { // vuln-code-snippet target-line testcodePathtraver019
        Ok(_) => super::shared::BenchmarkResponse::ok("Deleted"),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver019
