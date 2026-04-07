//! CWE-22: User-controlled filename passed directly to fs::write().

// vuln-code-snippet start testcodePathtraver018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");

    match std::fs::write(&filename, b"data") { // vuln-code-snippet target-line testcodePathtraver018
        Ok(_) => super::shared::BenchmarkResponse::ok("Written"),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver018
