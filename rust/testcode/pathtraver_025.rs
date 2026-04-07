//! CWE-22: User-controlled path passed to fs::metadata(), enabling information disclosure.

// vuln-code-snippet start testcodePathtraver025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");

    match std::fs::metadata(&path) { // vuln-code-snippet target-line testcodePathtraver025
        Ok(meta) => super::shared::BenchmarkResponse::ok(&format!("size={}", meta.len())),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver025
