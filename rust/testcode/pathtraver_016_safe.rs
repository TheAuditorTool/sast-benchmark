//! Path Traversal True Negative — CWE-22
//! Max path length enforced to limit traversal surface.

// vuln-code-snippet start testcodePathtraver016Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");
    let base = "/var/data";

    // SAFE: Enforce max path length
    if path.len() > 128 { // vuln-code-snippet safe-line testcodePathtraver016Safe
        return super::shared::BenchmarkResponse::bad_request("Path too long");
    }

    let full = format!("{}/{}", base, path);
    match std::fs::read_to_string(&full) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver016Safe
