pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount_str = req.param("amount");
    let amount: f64 = match amount_str.parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid amount"),
    };

    if amount <= 0.0 || amount >= 100_000.0 {
        return super::shared::BenchmarkResponse::bad_request("Amount must be between 0 and 100000");
    }

    let balance = 1000.0 - amount;
    super::shared::BenchmarkResponse::ok(&format!("Payment of {} processed. Balance: {}", amount, balance))
}
