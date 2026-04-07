use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    let val = COUNTER.load(Ordering::Relaxed);
    COUNTER.store(val + 1, Ordering::Relaxed);

    super::shared::BenchmarkResponse::ok(&format!("Counter: {}", val + 1))
}
