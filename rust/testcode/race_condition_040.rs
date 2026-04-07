//! CWE-362: Request counter incremented with single atomic fetch_add operation.

use std::sync::atomic::{AtomicU64, Ordering};

// vuln-code-snippet start testcodeRaceCondition040
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _key = req.param("key");
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let new_val = COUNTER.fetch_add(1, Ordering::SeqCst) + 1; // vuln-code-snippet target-line testcodeRaceCondition040
    super::shared::BenchmarkResponse::ok(&format!("Request #{}", new_val))
}
// vuln-code-snippet end testcodeRaceCondition040
