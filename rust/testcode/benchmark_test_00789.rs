pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let return_to = req.param("return_to");
    let location = format!("Location: {}", return_to);
    super::shared::BenchmarkResponse::ok(&location)
}
