//! CWE-352: Proper CSRF protection — Origin header validation before email update.

fn update_email(email: &str) -> bool {
    let _ = email;
    true
}

// vuln-code-snippet start testcodeCsrf034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if req.header("Origin") != "https://app.example.com" {
        return super::shared::BenchmarkResponse::forbidden("invalid origin");
    }
    let email = req.param("email");
    let result = update_email(&email); // vuln-code-snippet target-line testcodeCsrf034
    if result {
        super::shared::BenchmarkResponse::ok("email updated")
    } else {
        super::shared::BenchmarkResponse::error("update failed")
    }
}
// vuln-code-snippet end testcodeCsrf034
