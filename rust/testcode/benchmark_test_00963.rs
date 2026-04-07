pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw: i64 = req.param("quantity").parse().unwrap_or(0);
    let quantity = raw.clamp(1, 100);
    let total = quantity * 25;
    super::shared::BenchmarkResponse::ok(&format!("Total: {}", total))
}
