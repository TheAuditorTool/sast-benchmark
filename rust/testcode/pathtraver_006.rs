//! CWE-22: Reject if path contains ".." component.

// vuln-code-snippet start testcodePathtraver006
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");


    if path.contains("..") { // vuln-code-snippet target-line testcodePathtraver006
        return super::shared::BenchmarkResponse::forbidden("Path traversal blocked");
    }

    let full = format!("/var/data/{}", path);
    match std::fs::read_to_string(&full) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver006
