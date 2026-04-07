pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let cookie = make_session_cookie(&token);
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn make_session_cookie(token: &str) -> String {
    format!("session={}; Path=/; Max-Age=3600", token)
}
