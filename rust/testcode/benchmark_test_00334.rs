pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let base: i32 = req.param("base").parse().unwrap_or(2);
    let exp: u32 = req.param("exp").parse().unwrap_or(1);
    let result = base.pow(exp);
    super::shared::BenchmarkResponse::ok(&format!("Power: {}", result))
}
