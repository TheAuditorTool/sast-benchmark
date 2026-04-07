//! CWE-22: User-controlled path passed directly to fs::read_to_string().

// vuln-code-snippet start testcodePathtraver017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");

    match std::fs::read_to_string(&path) { // vuln-code-snippet target-line testcodePathtraver017
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver017
