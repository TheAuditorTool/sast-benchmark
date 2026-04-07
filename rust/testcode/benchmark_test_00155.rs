pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("value");
    let big_val: u64 = input.parse().unwrap_or(0);

    let small_val = big_val as u32;

    let msg = format!("Truncated {} -> {}", big_val, small_val);
    super::shared::BenchmarkResponse::ok(&msg)
}
