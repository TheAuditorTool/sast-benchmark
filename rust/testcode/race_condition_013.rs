//! CWE-362: Atomic compare-and-exchange with SeqCst ordering for counter update.

use std::sync::atomic::{AtomicU64, Ordering};

// vuln-code-snippet start testcodeRaceCondition013
static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    loop {
        let current = COUNTER.load(Ordering::SeqCst);
        match COUNTER.compare_exchange(current, current + 1, Ordering::SeqCst, Ordering::SeqCst) { // vuln-code-snippet target-line testcodeRaceCondition013
            Ok(_) => return super::shared::BenchmarkResponse::ok(&format!("Counter: {}", current + 1)),
            Err(_) => continue,
        }
    }
}
// vuln-code-snippet end testcodeRaceCondition013
