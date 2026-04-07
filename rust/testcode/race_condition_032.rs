//! CWE-362: Double-checked locking pattern without memory ordering guarantees allows race.

use std::sync::atomic::{AtomicBool, Ordering};

// vuln-code-snippet start testcodeRaceCondition032
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let key = req.param("key");
    static INITIALIZED: AtomicBool = AtomicBool::new(false);
    if !INITIALIZED.load(Ordering::Relaxed) {
        if !INITIALIZED.load(Ordering::Relaxed) { // vuln-code-snippet target-line testcodeRaceCondition032
            initialize_resource(&key);
            INITIALIZED.store(true, Ordering::Relaxed);
        }
    }
    super::shared::BenchmarkResponse::ok("Initialized")
}

fn initialize_resource(_key: &str) {}
// vuln-code-snippet end testcodeRaceCondition032
