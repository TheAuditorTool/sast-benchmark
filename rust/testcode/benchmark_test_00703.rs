use std::sync::atomic::{AtomicU64, Ordering};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _key = req.param("key");
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let new_val = COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    super::shared::BenchmarkResponse::ok(&format!("Request #{}", new_val))
}
