pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let big_val: u64 = req.param("value").parse().unwrap_or(0);
    let small = big_val as u32;
    super::shared::BenchmarkResponse::ok(&format!("Truncated: {}", small))
}
