fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn update_email(email: &str) -> bool {
    let _ = email;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let provided = req.header("X-CSRF-Token");
    let expected = req.cookie("csrf_session");
    if !constant_time_eq(provided.as_bytes(), expected.as_bytes()) {
        return super::shared::BenchmarkResponse::forbidden("csrf validation failed");
    }
    let email = req.param("email");
    let result = update_email(&email);
    if result {
        super::shared::BenchmarkResponse::ok("email updated")
    } else {
        super::shared::BenchmarkResponse::error("update failed")
    }
}
