//! Path Traversal True Positive — CWE-22
//! User filename in upload path allows ../ traversal.

// vuln-code-snippet start testcodePathtraver003Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_filename = req.param("filename");
    let base = "/var/www/uploads";

    // VULNERABLE: User filename directly interpolated into path
    let dest = format!("{}/{}", base, user_filename); // vuln-code-snippet vuln-line testcodePathtraver003Vulnerable

    match std::fs::write(&dest, req.body_str().as_bytes()) {
        Ok(_) => super::shared::BenchmarkResponse::ok("File uploaded"),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver003Vulnerable
