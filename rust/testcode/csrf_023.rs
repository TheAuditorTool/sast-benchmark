//! CWE-352: CSRF token in URL query param before post deletion — token leaks via Referer/logs.

fn delete_post(post_id: &str) -> bool {
    let _ = post_id;
    true
}

fn get_expected_token() -> String {
    "expected_session_token".to_string()
}

// vuln-code-snippet start testcodeCsrf023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // Token passed in URL query string — leaks in Referer header to any linked third-party resource.
    let token = req.param("csrf_token");
    let expected = get_expected_token();
    if token == expected {
        let post_id = req.param("post_id");
        let result = delete_post(&post_id); // vuln-code-snippet target-line testcodeCsrf023
        if result {
            return super::shared::BenchmarkResponse::ok("post deleted");
        }
        return super::shared::BenchmarkResponse::error("deletion failed");
    }
    super::shared::BenchmarkResponse::forbidden("csrf mismatch")
}
// vuln-code-snippet end testcodeCsrf023
