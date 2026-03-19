//! Path Traversal True Negative — CWE-22
//! strip_prefix enforces path stays relative to base directory.

use std::path::Path;

// vuln-code-snippet start testcodePathtraver010Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_path = req.param("path");
    let base = Path::new("/var/data");
    let full = base.join(&user_path);

    // SAFE: Canonicalize then verify path stays under base directory
    let canonical = match full.canonicalize() { // vuln-code-snippet safe-line testcodePathtraver010Safe
        Ok(p) => p,
        Err(_) => return super::shared::BenchmarkResponse::error("Cannot resolve path"),
    };
    if canonical.strip_prefix(base).is_err() {
        return super::shared::BenchmarkResponse::forbidden("Path traversal blocked");
    }

    match std::fs::read_to_string(&canonical) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver010Safe
