//! CWE-352: Proper CSRF protection — double-submit cookie with constant-time comparison before profile update.

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn update_profile(data: &str) -> bool {
    let _ = data;
    true
}

// vuln-code-snippet start testcodeCsrf040
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body_token = req.param("csrf_body");
    let cookie_token = req.cookie("csrf");
    if !constant_time_eq(body_token.as_bytes(), cookie_token.as_bytes()) {
        return super::shared::BenchmarkResponse::forbidden("csrf validation failed");
    }
    let data = req.param("data");
    let result = update_profile(&data); // vuln-code-snippet target-line testcodeCsrf040
    if result {
        super::shared::BenchmarkResponse::ok("profile updated")
    } else {
        super::shared::BenchmarkResponse::error("update failed")
    }
}
// vuln-code-snippet end testcodeCsrf040
