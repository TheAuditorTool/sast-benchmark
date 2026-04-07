use std::sync::atomic::{AtomicBool, Ordering};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let key = req.param("key");
    static INITIALIZED: AtomicBool = AtomicBool::new(false);
    if !INITIALIZED.load(Ordering::Relaxed) {
        if !INITIALIZED.load(Ordering::Relaxed) {
            initialize_resource(&key);
            INITIALIZED.store(true, Ordering::Relaxed);
        }
    }
    super::shared::BenchmarkResponse::ok("Initialized")
}

fn initialize_resource(_key: &str) {}
