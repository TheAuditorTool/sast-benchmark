pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dest = req.param("dest");
    let location = format!("Location: {}", dest);
    super::shared::BenchmarkResponse::ok(&location)
}
