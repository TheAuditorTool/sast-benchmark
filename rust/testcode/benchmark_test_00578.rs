pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut amount: i64 = req.param("amount").parse().unwrap_or(0);
    amount = 100;
    let result = process_payment(amount);
    super::shared::BenchmarkResponse::ok(&format!("ok={}", result))
}

fn process_payment(a: i64) -> i64 { a }
