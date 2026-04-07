//! CWE-22: Strict alphanumeric + dot/hyphen/underscore validation blocks traversal sequences.

// vuln-code-snippet start testcodePathtraver035
fn is_safe_filename(s: &str) -> bool {
    !s.contains("..") && !s.contains('/') && !s.contains('\\')
        && s.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-' || c == '_')
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("file");

    if !is_safe_filename(&filename) {
        return super::shared::BenchmarkResponse::forbidden("Invalid filename");
    }

    let path = format!("/uploads/{}", filename);
    match std::fs::read_to_string(&path) { // vuln-code-snippet target-line testcodePathtraver035
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver035
