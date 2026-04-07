//! CWE-352: CSRF token in URL query param for password reset — token leaks via Referer/logs.

fn reset_password(user_id: &str, new_pwd: &str) -> bool {
    let _ = (user_id, new_pwd);
    true
}

fn get_expected_token() -> String {
    "expected_session_token".to_string()
}

// vuln-code-snippet start testcodeCsrf025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // CSRF token embedded in URL — visible in browser history and Referer leakage to analytics.
    let token = req.param("csrf_token");
    let expected = get_expected_token();
    if token == expected {
        let user_id = req.param("user_id");
        let new_pwd = req.param("new_pwd");
        let result = reset_password(&user_id, &new_pwd); // vuln-code-snippet target-line testcodeCsrf025
        if result {
            return super::shared::BenchmarkResponse::ok("password reset");
        }
        return super::shared::BenchmarkResponse::error("reset failed");
    }
    super::shared::BenchmarkResponse::forbidden("csrf mismatch")
}
// vuln-code-snippet end testcodeCsrf025
