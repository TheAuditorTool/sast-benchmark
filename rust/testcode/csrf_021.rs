//! CWE-352: CSRF token passed in URL query parameter before settings change — leaks via Referer/logs.

fn change_settings(data: &str) -> bool {
    let _ = data;
    true
}

fn get_expected_token() -> String {
    "expected_session_token".to_string()
}

// vuln-code-snippet start testcodeCsrf021
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // Token sourced from URL query param — exposed in server logs, Referer header, and browser history.
    let tok = req.param("csrf_token");
    let expected = get_expected_token();
    if tok == expected {
        let data = req.param("data");
        let result = change_settings(&data); // vuln-code-snippet target-line testcodeCsrf021
        if result {
            return super::shared::BenchmarkResponse::ok("settings changed");
        }
        return super::shared::BenchmarkResponse::error("change failed");
    }
    super::shared::BenchmarkResponse::forbidden("csrf mismatch")
}
// vuln-code-snippet end testcodeCsrf021
