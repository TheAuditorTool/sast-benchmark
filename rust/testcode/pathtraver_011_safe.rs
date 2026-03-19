//! Path Traversal True Negative — CWE-22
//! Chroot-style: canonicalize then verify prefix.

use std::path::Path;

// vuln-code-snippet start testcodePathtraver011Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_path = req.param("path");
    let base_dir = "/var/data";

    let base = match Path::new(base_dir).canonicalize() {
        Ok(p) => p,
        Err(e) => return super::shared::BenchmarkResponse::error(&e.to_string()),
    };
    let full = match base.join(&user_path).canonicalize() {
        Ok(p) => p,
        Err(e) => return super::shared::BenchmarkResponse::error(&e.to_string()),
    };

    // SAFE: Canonicalize resolves symlinks/.. then check prefix
    if !full.starts_with(&base) { // vuln-code-snippet safe-line testcodePathtraver011Safe
        return super::shared::BenchmarkResponse::forbidden("Path traversal blocked");
    }

    match std::fs::read_to_string(&full) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver011Safe
