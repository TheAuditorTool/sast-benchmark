//! CWE-352: Proper CSRF protection — constant-time token comparison before password change.

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn change_password(new_password: &str) -> bool {
    let _ = new_password;
    true
}

// vuln-code-snippet start testcodeCsrf026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let provided = req.header("X-CSRF-Token");
    let expected = req.cookie("csrf_session");
    if !constant_time_eq(provided.as_bytes(), expected.as_bytes()) {
        return super::shared::BenchmarkResponse::forbidden("csrf validation failed");
    }
    let new_password = req.param("new_password");
    let result = change_password(&new_password); // vuln-code-snippet target-line testcodeCsrf026
    if result {
        super::shared::BenchmarkResponse::ok("password changed")
    } else {
        super::shared::BenchmarkResponse::error("change failed")
    }
}
// vuln-code-snippet end testcodeCsrf026
