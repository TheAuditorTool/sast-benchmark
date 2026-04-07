//! CWE-352: CSRF token comparison result discarded before profile update.

fn csrf_token_matches(provided: &str, expected: &str) -> bool {
    provided == expected
}

fn update_profile(data: &str) -> bool {
    let _ = data;
    true
}

// vuln-code-snippet start testcodeCsrf007
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _ = csrf_token_matches(&req.param("csrf"), "session_token"); // result discarded
    let data = req.param("data");
    let result = update_profile(&data); // vuln-code-snippet target-line testcodeCsrf007
    if result {
        super::shared::BenchmarkResponse::ok("profile updated")
    } else {
        super::shared::BenchmarkResponse::error("update failed")
    }
}
// vuln-code-snippet end testcodeCsrf007
