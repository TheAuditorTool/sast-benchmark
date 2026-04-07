//! CWE-352: Any non-empty X-CSRF-Token header passes — not validated against session.

fn update_email(email: &str) -> bool {
    let _ = email;
    true
}

// vuln-code-snippet start testcodeCsrf012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if req.header("X-CSRF-Token").len() > 0 {
        let email = req.param("email");
        let result = update_email(&email); // vuln-code-snippet target-line testcodeCsrf012
        if result {
            return super::shared::BenchmarkResponse::ok("email updated");
        }
        return super::shared::BenchmarkResponse::error("update failed");
    }
    super::shared::BenchmarkResponse::bad_request("missing X-CSRF-Token header")
}
// vuln-code-snippet end testcodeCsrf012
