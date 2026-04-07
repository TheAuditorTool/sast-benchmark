pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let count: u32 = req.param("count").parse().unwrap_or(0);
    let increment: u32 = req.param("step").parse().unwrap_or(1);
    let mut total: u32 = 0;
    for _ in 0..count {
        total += increment;
    }
    super::shared::BenchmarkResponse::ok(&format!("total={}", total))
}
