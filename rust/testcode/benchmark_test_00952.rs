use std::sync::atomic::{AtomicI64, Ordering};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: i64 = req.param("amount").parse().unwrap_or(0);
    static BALANCE: AtomicI64 = AtomicI64::new(1000);
    let prev = BALANCE.fetch_sub(amount, Ordering::SeqCst);
    if prev >= amount {
        super::shared::BenchmarkResponse::ok("Transfer complete")
    } else {
        BALANCE.fetch_add(amount, Ordering::SeqCst);
        super::shared::BenchmarkResponse::bad_request("Insufficient funds")
    }
}
