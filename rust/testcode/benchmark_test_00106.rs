pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let next = req.param("next");
    let location = format!("Location: {}", next);
    super::shared::BenchmarkResponse::ok(&location)
}
