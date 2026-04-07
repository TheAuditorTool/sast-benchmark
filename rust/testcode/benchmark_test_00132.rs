use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = COUNTER.fetch_add(1, Ordering::SeqCst);

    super::shared::BenchmarkResponse::ok(&format!("Token: {:016x}", token))
}
