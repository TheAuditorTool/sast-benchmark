//! CWE-362: Atomic counter incremented via separate load and store operations.

use std::sync::atomic::{AtomicU64, Ordering};

// vuln-code-snippet start testcodeRaceCondition007
static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    let val = COUNTER.load(Ordering::Relaxed);
    COUNTER.store(val + 1, Ordering::Relaxed); // vuln-code-snippet target-line testcodeRaceCondition007

    super::shared::BenchmarkResponse::ok(&format!("Counter: {}", val + 1))
}
// vuln-code-snippet end testcodeRaceCondition007
