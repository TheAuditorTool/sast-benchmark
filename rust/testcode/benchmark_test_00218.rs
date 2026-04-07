use std::sync::atomic::{AtomicBool, Ordering};

static READY: AtomicBool = AtomicBool::new(false);
static mut SHARED_DATA: u64 = 0;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    if READY.load(Ordering::Relaxed) {
        let val = unsafe { SHARED_DATA };
        super::shared::BenchmarkResponse::ok(&format!("Data: {}", val))
    } else {
        super::shared::BenchmarkResponse::ok("Not ready")
    }
}
