pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let value = req.param("session_id");

    let cookie = cookie_new("auth", &value);

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn cookie_new(name: &str, value: &str) -> String {
    format!("{}={}", name, value)
}
