pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    super::shared::BenchmarkResponse::ok(&format!("Registered: {}", username))
}
