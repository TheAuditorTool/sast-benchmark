pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    let csrf_token = "a1b2c3d4e5f6";
    let cookie = format!("csrf={}; Path=/; Secure; SameSite=Strict", csrf_token);

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
