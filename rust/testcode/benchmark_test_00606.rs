pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let large: u64 = req.param("value").parse().unwrap_or(0);
    let truncated = large as u32;
    super::shared::BenchmarkResponse::ok(&format!("truncated={}", truncated))
}
