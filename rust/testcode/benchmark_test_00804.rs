pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: u64 = req.param("a").parse().unwrap_or(0);
    let b: u64 = req.param("b").parse().unwrap_or(0);

    let result = a.saturating_add(b);

    super::shared::BenchmarkResponse::ok(&format!("Sum: {}", result))
}
