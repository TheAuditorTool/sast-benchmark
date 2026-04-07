pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let body = format!("Welcome: {}", name);

    super::shared::BenchmarkResponse::ok(&body)
}
