//! CWE-362: Compile-time constant ensures atomic operation always used; TOCTOU branch unreachable.

use std::sync::atomic::{AtomicU64, Ordering};

// vuln-code-snippet start testcodeRaceCondition049
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _key = req.param("key");
    static CTR: AtomicU64 = AtomicU64::new(0);
    let val = if 100 > 50 {
        CTR.fetch_add(1, Ordering::SeqCst) + 1 // vuln-code-snippet target-line testcodeRaceCondition049
    } else {
        let v = CTR.load(Ordering::Relaxed);
        CTR.store(v + 1, Ordering::Relaxed);
        v + 1
    };
    super::shared::BenchmarkResponse::ok(&format!("Count: {}", val))
}
// vuln-code-snippet end testcodeRaceCondition049
