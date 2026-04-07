//! CWE-352: Proper CSRF protection — constant-time token comparison before account deletion.

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn delete_account(user_id: &str) -> bool {
    let _ = user_id;
    true
}

// vuln-code-snippet start testcodeCsrf028
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let provided = req.header("X-CSRF-Token");
    let expected = req.cookie("csrf_session");
    if !constant_time_eq(provided.as_bytes(), expected.as_bytes()) {
        return super::shared::BenchmarkResponse::forbidden("csrf validation failed");
    }
    let user_id = req.param("user_id");
    let result = delete_account(&user_id); // vuln-code-snippet target-line testcodeCsrf028
    if result {
        super::shared::BenchmarkResponse::ok("account deleted")
    } else {
        super::shared::BenchmarkResponse::error("deletion failed")
    }
}
// vuln-code-snippet end testcodeCsrf028
