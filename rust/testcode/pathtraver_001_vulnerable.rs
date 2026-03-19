//! Path Traversal True Positive — CWE-22
//! User-controlled file path passed directly to fs::read_to_string().
//! No validation — ../../etc/passwd is possible.

// vuln-code-snippet start testcodePathtraver001Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");

    // VULNERABLE: User controls file path with no validation
    match std::fs::read_to_string(&path) { // vuln-code-snippet vuln-line testcodePathtraver001Vulnerable
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver001Vulnerable
