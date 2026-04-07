//! CWE-352: Structural TN — read-only GET operation, no state mutation, CSRF not applicable.

fn fetch_profile(user_id: &str) -> String {
    let _ = user_id;
    "profile_data".to_string()
}

// vuln-code-snippet start testcodeCsrf046
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // Read-only operation — no state mutation, CSRF not applicable.
    let user_id = req.param("user_id");
    let user_data = fetch_profile(&user_id); // vuln-code-snippet target-line testcodeCsrf046
    super::shared::BenchmarkResponse::ok(&user_data)
}
// vuln-code-snippet end testcodeCsrf046
