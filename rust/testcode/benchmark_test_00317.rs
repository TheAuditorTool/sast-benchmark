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

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body_token = req.param("csrf_body");
    let cookie_token = req.cookie("csrf");
    if !constant_time_eq(body_token.as_bytes(), cookie_token.as_bytes()) {
        return super::shared::BenchmarkResponse::forbidden("csrf validation failed");
    }
    let id = req.param("id");
    let result = delete_resource(&id);
    if result {
        super::shared::BenchmarkResponse::ok("resource deleted")
    } else {
        super::shared::BenchmarkResponse::error("deletion failed")
    }
}
