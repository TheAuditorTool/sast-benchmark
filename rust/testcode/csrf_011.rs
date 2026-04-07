//! CWE-352: CSRF token presence-only check (non-empty) before password change.

fn change_password(new_password: &str) -> bool {
    let _ = new_password;
    true
}

// vuln-code-snippet start testcodeCsrf011
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if req.param("csrf_token").is_empty() {
        return super::shared::BenchmarkResponse::bad_request("missing csrf token");
    }
    // Any non-empty string passes — not validated against the stored session token.
    let new_password = req.param("new_password");
    let result = change_password(&new_password); // vuln-code-snippet target-line testcodeCsrf011
    if result {
        super::shared::BenchmarkResponse::ok("password changed")
    } else {
        super::shared::BenchmarkResponse::error("change failed")
    }
}
// vuln-code-snippet end testcodeCsrf011
