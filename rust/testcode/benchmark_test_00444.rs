pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    let cookie = format!("session={}; Path=/; Max-Age=900; Secure; HttpOnly; SameSite=Strict", token);

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
