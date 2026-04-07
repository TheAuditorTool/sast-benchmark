pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw_base = req.param("base");
    let raw_exp = req.param("exp");
    let base: u64 = raw_base.parse().unwrap_or(2);
    let exp: u32 = raw_exp.parse().unwrap_or(1);
    let result: u64 = base << exp;
    super::shared::BenchmarkResponse::ok(&format!("Result: {}", result))
}
