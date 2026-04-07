pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: u64 = req.param("amount").parse().unwrap_or(0);
    let cost: u64 = req.param("cost").parse().unwrap_or(0);
    let result = amount.wrapping_sub(cost);
    super::shared::BenchmarkResponse::ok(&format!("Balance: {}", result))
}
