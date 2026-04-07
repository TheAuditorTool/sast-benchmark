//! CWE-22: Prefixing with a base directory does not prevent "../" traversal above it.

// vuln-code-snippet start testcodePathtraver023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");
    let full = format!("/app/static/{}", path);

    match std::fs::read_to_string(&full) { // vuln-code-snippet target-line testcodePathtraver023
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver023
