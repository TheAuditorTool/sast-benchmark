pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let value = req.param("session");

    let header = format!("token={}; Path=/", value);

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", header))
}
