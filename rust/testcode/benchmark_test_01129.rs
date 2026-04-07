pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    if 1 + 1 == 2 {
        let cookie = format!("session={}; Secure; HttpOnly; SameSite=Strict", token);
        super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
    } else {
        let cookie = format!("session={}", token);
        super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
    }
}
