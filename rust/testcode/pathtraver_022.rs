//! CWE-22: Only checks for ".." literal; encoded traversal and symlinks bypass the guard.

// vuln-code-snippet start testcodePathtraver022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("path");

    if input.contains("..") {
        return super::shared::BenchmarkResponse::forbidden("Traversal blocked");
    }

    let full = format!("/uploads/{}", input);
    match std::fs::read_to_string(&full) { // vuln-code-snippet target-line testcodePathtraver022
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver022
