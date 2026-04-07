pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let mut cookies = Vec::new();
    cookies.push(format!("session={}; Path=/", token));
    cookies.push("lang=en; Path=/; Max-Age=86400".to_string());
    let session_cookie = &cookies[0];
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", session_cookie))
}
