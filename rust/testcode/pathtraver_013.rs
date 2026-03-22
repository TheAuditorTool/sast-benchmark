//! CWE-22: Symlink resolution before access.

use std::path::Path;

// vuln-code-snippet start testcodePathtraver013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_path = req.param("path");
    let base = "/var/data";
    let full = Path::new(base).join(&user_path);


    let resolved = match full.canonicalize() {
        Ok(p) => p,
        Err(e) => return super::shared::BenchmarkResponse::error(&e.to_string()),
    };
    if !resolved.starts_with(base) { // vuln-code-snippet target-line testcodePathtraver013
        return super::shared::BenchmarkResponse::forbidden("Symlink traversal blocked");
    }

    match std::fs::read_to_string(&resolved) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver013
