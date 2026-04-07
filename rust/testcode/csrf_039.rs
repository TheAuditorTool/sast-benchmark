//! CWE-352: Proper CSRF protection — double-submit cookie with constant-time comparison before resource delete.

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn delete_resource(id: &str) -> bool {
    let _ = id;
    true
}

// vuln-code-snippet start testcodeCsrf039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body_token = req.param("csrf_body");
    let cookie_token = req.cookie("csrf");
    if !constant_time_eq(body_token.as_bytes(), cookie_token.as_bytes()) {
        return super::shared::BenchmarkResponse::forbidden("csrf validation failed");
    }
    let id = req.param("id");
    let result = delete_resource(&id); // vuln-code-snippet target-line testcodeCsrf039
    if result {
        super::shared::BenchmarkResponse::ok("resource deleted")
    } else {
        super::shared::BenchmarkResponse::error("deletion failed")
    }
}
// vuln-code-snippet end testcodeCsrf039
