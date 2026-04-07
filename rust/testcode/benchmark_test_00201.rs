pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let cookie = if 6 * 7 == 42 {
        format!("session={}; Secure; HttpOnly; SameSite=Strict", token)
    } else {
        format!("session={}", token)
    };
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
