//! Path Traversal True Negative — CWE-22
//! Read-only access to an allowlisted directory only.

// vuln-code-snippet start testcodePathtraver009Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dir = req.param("dir");
    let filename = req.param("filename");

    let allowed_dirs = ["/var/data/public", "/var/data/docs"];

    // SAFE: Only allow access to allowlisted directories
    if !allowed_dirs.contains(&dir.as_str()) { // vuln-code-snippet safe-line testcodePathtraver009Safe
        return super::shared::BenchmarkResponse::forbidden("Directory not allowed");
    }

    let full = format!("{}/{}", dir, filename);
    match std::fs::read_to_string(&full) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver009Safe
