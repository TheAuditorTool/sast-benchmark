//! CWE-362: AtomicBool with Relaxed ordering used as inter-thread synchronization flag.

use std::sync::atomic::{AtomicBool, Ordering};

// vuln-code-snippet start testcodeRaceCondition005
static READY: AtomicBool = AtomicBool::new(false);
static mut SHARED_DATA: u64 = 0;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    if READY.load(Ordering::Relaxed) { // vuln-code-snippet target-line testcodeRaceCondition005
        let val = unsafe { SHARED_DATA };
        super::shared::BenchmarkResponse::ok(&format!("Data: {}", val))
    } else {
        super::shared::BenchmarkResponse::ok("Not ready")
    }
}
// vuln-code-snippet end testcodeRaceCondition005
