use std::sync::Mutex;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: i64 = req.param("amount").parse().unwrap_or(0);
    static BALANCE: Mutex<i64> = Mutex::new(1000);
    let mut guard = BALANCE.lock().unwrap();
    if *guard >= amount {
        *guard -= amount;
        super::shared::BenchmarkResponse::ok("Deducted")
    } else {
        super::shared::BenchmarkResponse::bad_request("Insufficient")
    }
}
