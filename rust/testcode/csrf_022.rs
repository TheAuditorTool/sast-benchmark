//! CWE-352: CSRF token in URL query param before profile update — token leaks in Referer/logs.

fn update_profile(data: &str) -> bool {
    let _ = data;
    true
}

fn get_expected_token() -> String {
    "expected_session_token".to_string()
}

// vuln-code-snippet start testcodeCsrf022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // Token sourced from URL — visible in server access logs and Referer headers to third parties.
    let csrf_token = req.param("csrf_token");
    let expected = get_expected_token();
    if csrf_token == expected {
        let data = req.param("data");
        let result = update_profile(&data); // vuln-code-snippet target-line testcodeCsrf022
        if result {
            return super::shared::BenchmarkResponse::ok("profile updated");
        }
        return super::shared::BenchmarkResponse::error("update failed");
    }
    super::shared::BenchmarkResponse::forbidden("csrf mismatch")
}
// vuln-code-snippet end testcodeCsrf022
