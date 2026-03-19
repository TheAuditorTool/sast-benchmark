//! Path Traversal True Negative — CWE-22
//! Read-only access to an allowlisted directory only.

// vuln-code-snippet start testcodePathtraver009Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dir = req.param("dir");
    let filename = req.param("filename");

    let allowed_dirs = ["/var/data/public", "/var/data/docs"];

    // Only allow access to allowlisted directories
    if !allowed_dirs.contains(&dir.as_str()) {
        return super::shared::BenchmarkResponse::forbidden("Directory not allowed");
    }

    // SAFE: Reject traversal and path separator characters in filename
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') { // vuln-code-snippet safe-line testcodePathtraver009Safe
        return super::shared::BenchmarkResponse::forbidden("Invalid filename");
    }

    let full = format!("{}/{}", dir, filename);
    match std::fs::read_to_string(&full) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver009Safe
