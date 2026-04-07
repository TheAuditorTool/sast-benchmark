pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: i32 = req.param("value").parse().unwrap_or(0);
    let negated = -val;
    super::shared::BenchmarkResponse::ok(&format!("negated={}", negated))
}
