//! CWE-352: CSRF token from wrong source — body param checked against itself, cookie mismatch undetected.

fn post_comment(comment: &str) -> bool {
    let _ = comment;
    true
}

// vuln-code-snippet start testcodeCsrf010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // Checks body param is non-empty, but the authoritative token lives in the session cookie.
    // An attacker-controlled form can supply any value for req.param("csrf").
    if req.param("csrf") != "" {
        let comment = req.param("comment");
        let result = post_comment(&comment); // vuln-code-snippet target-line testcodeCsrf010
        if result {
            return super::shared::BenchmarkResponse::ok("comment posted");
        }
    }
    super::shared::BenchmarkResponse::bad_request("missing csrf token")
}
// vuln-code-snippet end testcodeCsrf010
