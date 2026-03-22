//! CWE-22: Max path length enforced.

// vuln-code-snippet start testcodePathtraver016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");
    let base = "/var/data";

    // Enforce max path length
    if path.len() > 128 {
        return super::shared::BenchmarkResponse::bad_request("Path too long");
    }


    if path.contains("..") || path.contains('/') || path.contains('\\') { // vuln-code-snippet target-line testcodePathtraver016
        return super::shared::BenchmarkResponse::forbidden("Path traversal characters blocked");
    }

    let full = format!("{}/{}", base, path);
    match std::fs::read_to_string(&full) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver016
