pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: i64 = req.param("a").parse().unwrap_or(0);
    let b: i64 = req.param("b").parse().unwrap_or(0);
    let sum = a + b;
    super::shared::BenchmarkResponse::ok(&format!("sum={}", sum))
}
