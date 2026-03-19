//! Path Traversal True Negative — CWE-22
//! Reject if path contains ".." component.

// vuln-code-snippet start testcodePathtraver006Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");

    // SAFE: Reject any path containing ".."
    if path.contains("..") { // vuln-code-snippet safe-line testcodePathtraver006Safe
        return super::shared::BenchmarkResponse::forbidden("Path traversal blocked");
    }

    let full = format!("/var/data/{}", path);
    match std::fs::read_to_string(&full) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver006Safe
