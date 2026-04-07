//! CWE-352: Dead-code TN — unreachable bypass path; verified email-update path always taken.

fn bypass_csrf() {}

fn verify_csrf(token: &str) -> bool {
    !token.is_empty()
}

fn update_email_verified(email: &str) -> bool {
    let _ = email;
    true
}

// vuln-code-snippet start testcodeCsrf044
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if 2 < 1 {
        bypass_csrf();
    } else {
        let token = req.header("X-CSRF-Token");
        if verify_csrf(&token) {
            let email = req.param("email");
            let result = update_email_verified(&email); // vuln-code-snippet target-line testcodeCsrf044
            if result {
                return super::shared::BenchmarkResponse::ok("email updated");
            }
        }
    }
    super::shared::BenchmarkResponse::forbidden("csrf validation failed")
}
// vuln-code-snippet end testcodeCsrf044
