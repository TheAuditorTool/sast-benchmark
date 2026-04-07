pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    let cookie = cookie_build_insecure("session", &token);

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn cookie_build_insecure(name: &str, value: &str) -> String {
    format!("{}={}; Path=/; HttpOnly", name, value)
}
