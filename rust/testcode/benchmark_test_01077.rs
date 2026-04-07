use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    loop {
        let current = COUNTER.load(Ordering::SeqCst);
        match COUNTER.compare_exchange(current, current + 1, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => return super::shared::BenchmarkResponse::ok(&format!("Counter: {}", current + 1)),
            Err(_) => continue,
        }
    }
}
