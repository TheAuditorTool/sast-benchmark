//! CWE-352: Non-constant-time CSRF check before group membership change (timing oracle).

fn add_user_to_group(user_id: &str, group: &str) -> bool {
    let _ = (user_id, group);
    true
}

fn get_expected_token() -> String {
    "expected_session_token".to_string()
}

// vuln-code-snippet start testcodeCsrf020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let expected_token = get_expected_token();
    // == short-circuits, allowing a timing side-channel attack.
    if req.param("csrf_token") == expected_token {
        let user_id = req.param("user_id");
        let group = req.param("group");
        let result = add_user_to_group(&user_id, &group); // vuln-code-snippet target-line testcodeCsrf020
        if result {
            return super::shared::BenchmarkResponse::ok("user added to group");
        }
        return super::shared::BenchmarkResponse::error("operation failed");
    }
    super::shared::BenchmarkResponse::forbidden("csrf mismatch")
}
// vuln-code-snippet end testcodeCsrf020
