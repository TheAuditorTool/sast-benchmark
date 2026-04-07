use std::sync::atomic::{AtomicBool, Ordering};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _key = req.param("key");
    static ACTIVE: AtomicBool = AtomicBool::new(false);
    let was_active = ACTIVE.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst);
    match was_active {
        Ok(_) => super::shared::BenchmarkResponse::ok("Activated"),
        Err(_) => super::shared::BenchmarkResponse::bad_request("Already active"),
    }
}
