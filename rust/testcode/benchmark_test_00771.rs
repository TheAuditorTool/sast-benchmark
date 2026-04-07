pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: i32 = req.param("a").parse().unwrap_or(0);
    let b: i32 = req.param("b").parse().unwrap_or(0);
    let result = a * b;
    super::shared::BenchmarkResponse::ok(&format!("result={}", result))
}
