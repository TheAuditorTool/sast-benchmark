fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn assign_role(user_id: &str, role: &str) -> bool {
    let _ = (user_id, role);
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let provided = req.header("X-CSRF-Token");
    let expected = req.cookie("csrf_session");
    if !constant_time_eq(provided.as_bytes(), expected.as_bytes()) {
        return super::shared::BenchmarkResponse::forbidden("csrf validation failed");
    }
    let user_id = req.param("user_id");
    let role = req.param("role");
    let result = assign_role(&user_id, &role);
    if result {
        super::shared::BenchmarkResponse::ok("role assigned")
    } else {
        super::shared::BenchmarkResponse::error("role assignment failed")
    }
}
