pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let idx: usize = req.param("index").parse().unwrap_or(0);
    let data = [10, 20, 30, 40, 50];
    let value = data[idx];
    super::shared::BenchmarkResponse::ok(&format!("Value: {}", value))
}
