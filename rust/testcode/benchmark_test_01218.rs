pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.param("n");
    let n: usize = raw.parse::<usize>().unwrap_or(1).min(100);
    let items: Vec<u64> = (0..n as u64).collect();
    let sum: u64 = items.iter().sum();
    super::shared::BenchmarkResponse::ok(&format!("Sum of {} items: {}", n, sum))
}
