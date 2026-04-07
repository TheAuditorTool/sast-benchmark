pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let value: u32 = req.param("value").parse().unwrap_or(1);
    let shift: u32 = req.param("shift").parse().unwrap_or(0).min(31);
    let result = value << shift;
    super::shared::BenchmarkResponse::ok(&format!("result={}", result))
}
