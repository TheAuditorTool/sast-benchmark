//! Path Traversal True Negative — CWE-22
//! Path canonicalized and validated against base directory before reading.
//! Isomorphic to pathtraver_001_vulnerable — same parameter, safe validation.

use std::path::Path;

// vuln-code-snippet start testcodePathtraver002Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_path = req.param("path");
    let base_dir = "/var/data";

    // SAFE: Canonicalize and validate path stays within base directory
    let base = match Path::new(base_dir).canonicalize() {
        Ok(p) => p,
        Err(e) => return super::shared::BenchmarkResponse::error(&e.to_string()),
    };
    let full = base.join(&user_path);
    let canonical = match full.canonicalize() {
        Ok(p) => p,
        Err(e) => return super::shared::BenchmarkResponse::error(&e.to_string()),
    };
    if !canonical.starts_with(&base) { // vuln-code-snippet safe-line testcodePathtraver002Safe
        return super::shared::BenchmarkResponse::forbidden("Path traversal blocked");
    }

    match std::fs::read_to_string(&canonical) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver002Safe
