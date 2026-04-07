pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let numerator: i64 = req.param("num").parse().unwrap_or(0);
    let denominator: i64 = req.param("den").parse().unwrap_or(1);
    let result = (numerator / denominator) as i16;
    super::shared::BenchmarkResponse::ok(&format!("result={}", result))
}
