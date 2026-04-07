pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: i64 = req.param("amount").parse().unwrap_or(0);
    let result = process_payment(amount);
    super::shared::BenchmarkResponse::ok(&format!("processed={}", result))
}

fn process_payment(amount: i64) -> i64 { amount }
