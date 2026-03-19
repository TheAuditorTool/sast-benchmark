//! Path Traversal True Positive — CWE-22
//! User controls directory creation path with no validation.

// vuln-code-snippet start testcodePathtraver004Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_path = req.param("dir");

    // VULNERABLE: User controls directory creation path
    match std::fs::create_dir_all(&user_path) { // vuln-code-snippet vuln-line testcodePathtraver004Vulnerable
        Ok(_) => super::shared::BenchmarkResponse::ok("Directory created"),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver004Vulnerable
