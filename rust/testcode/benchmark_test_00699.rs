pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    let cookie = build_session_cookie(&token);

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn build_session_cookie(value: &str) -> String {
    format!("session={}; Path=/; HttpOnly", value)
}
