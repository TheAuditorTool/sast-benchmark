//! Path Traversal True Negative — CWE-22
//! Max path length enforced to limit traversal surface.

// vuln-code-snippet start testcodePathtraver016Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");
    let base = "/var/data";

    // Enforce max path length
    if path.len() > 128 {
        return super::shared::BenchmarkResponse::bad_request("Path too long");
    }

    // SAFE: Reject traversal and path separator characters
    if path.contains("..") || path.contains('/') || path.contains('\\') { // vuln-code-snippet safe-line testcodePathtraver016Safe
        return super::shared::BenchmarkResponse::forbidden("Path traversal characters blocked");
    }

    let full = format!("{}/{}", base, path);
    match std::fs::read_to_string(&full) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver016Safe
