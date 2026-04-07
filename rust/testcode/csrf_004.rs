//! CWE-352: Email update with no CSRF token check.

fn update_email(email: &str) -> bool {
    let _ = email;
    true
}

// vuln-code-snippet start testcodeCsrf004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    let result = update_email(&email); // vuln-code-snippet target-line testcodeCsrf004
    if result {
        super::shared::BenchmarkResponse::ok("email updated")
    } else {
        super::shared::BenchmarkResponse::error("update failed")
    }
}
// vuln-code-snippet end testcodeCsrf004
