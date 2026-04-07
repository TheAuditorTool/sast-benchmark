pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let shift: u32 = req.param("shift").parse().unwrap_or(0);
    let result = 1u32 << shift;
    super::shared::BenchmarkResponse::ok(&format!("Shifted: {}", result))
}
