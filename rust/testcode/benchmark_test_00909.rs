pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount_str = req.param("amount");

    let amount: f64 = amount_str.parse().unwrap_or(0.0);
    let balance = 1000.0 - amount;

    super::shared::BenchmarkResponse::ok(&format!("Payment of {} processed. Balance: {}", amount, balance))
}
