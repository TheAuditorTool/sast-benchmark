//! CWE-22: Length-only check does not prevent path traversal sequences.

// vuln-code-snippet start testcodePathtraver027
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");

    if path.len() > 200 {
        return super::shared::BenchmarkResponse::bad_request("Path too long");
    }

    let full = format!("/uploads/{}", path);
    match std::fs::read_to_string(&full) { // vuln-code-snippet target-line testcodePathtraver027
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver027
