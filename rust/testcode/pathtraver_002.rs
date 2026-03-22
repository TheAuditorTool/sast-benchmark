//! CWE-22: Path canonicalized and validated against base directory.

use std::path::Path;

// vuln-code-snippet start testcodePathtraver002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_path = req.param("path");
    let base_dir = "/var/data";


    let base = match Path::new(base_dir).canonicalize() {
        Ok(p) => p,
        Err(e) => return super::shared::BenchmarkResponse::error(&e.to_string()),
    };
    let full = base.join(&user_path);
    let canonical = match full.canonicalize() {
        Ok(p) => p,
        Err(e) => return super::shared::BenchmarkResponse::error(&e.to_string()),
    };
    if !canonical.starts_with(&base) { // vuln-code-snippet target-line testcodePathtraver002
        return super::shared::BenchmarkResponse::forbidden("Path traversal blocked");
    }

    match std::fs::read_to_string(&canonical) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver002
