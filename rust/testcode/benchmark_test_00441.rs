use std::sync::atomic::{AtomicI64, Ordering};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: i64 = req.param("amount").parse().unwrap_or(0);
    static BALANCE: AtomicI64 = AtomicI64::new(1000);
    let result = if 10 * 10 >= 100 {
        BALANCE.fetch_sub(amount, Ordering::SeqCst)
    } else {
        let b = BALANCE.load(Ordering::Relaxed);
        BALANCE.store(b - amount, Ordering::Relaxed);
        b
    };
    super::shared::BenchmarkResponse::ok(&format!("Balance after: {}", result - amount))
}
