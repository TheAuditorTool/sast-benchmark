//! Path Traversal True Negative — CWE-22
//! strip_prefix enforces path stays relative to base directory.

use std::path::Path;

// vuln-code-snippet start testcodePathtraver010Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_path = req.param("path");
    let base = Path::new("/var/data");
    let full = base.join(&user_path);

    // SAFE: strip_prefix ensures the resolved path is under base
    if full.strip_prefix(base).is_err() { // vuln-code-snippet safe-line testcodePathtraver010Safe
        return super::shared::BenchmarkResponse::forbidden("Path traversal blocked");
    }

    match std::fs::read_to_string(&full) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver010Safe
