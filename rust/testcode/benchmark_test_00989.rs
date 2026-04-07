use std::collections::HashMap;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut cookies = HashMap::new();
    let token = req.param("token");
    cookies.insert("raw", format!("session={}", token));
    cookies.insert("secure", format!("session={}; Secure; HttpOnly; SameSite=Strict", token));
    let cookie = cookies.get("secure").unwrap();
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
