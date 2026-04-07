//! CWE-362: Atomic compare-exchange ensures only one thread sets the initialization flag.

use std::sync::atomic::{AtomicBool, Ordering};

// vuln-code-snippet start testcodeRaceCondition038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _key = req.param("key");
    static ACTIVE: AtomicBool = AtomicBool::new(false);
    let was_active = ACTIVE.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst); // vuln-code-snippet target-line testcodeRaceCondition038
    match was_active {
        Ok(_) => super::shared::BenchmarkResponse::ok("Activated"),
        Err(_) => super::shared::BenchmarkResponse::bad_request("Already active"),
    }
}
// vuln-code-snippet end testcodeRaceCondition038
