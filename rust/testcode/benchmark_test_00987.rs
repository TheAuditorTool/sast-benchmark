pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let width: u32 = req.param("width").parse().unwrap_or(0);
    let height: u32 = req.param("height").parse().unwrap_or(0);
    let pixels = width.saturating_mul(height);
    super::shared::BenchmarkResponse::ok(&format!("Pixels: {}", pixels))
}
