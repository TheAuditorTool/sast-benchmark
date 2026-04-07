pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    let cookie = format!("__Host-session={}; Path=/; Secure; HttpOnly", token);

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
