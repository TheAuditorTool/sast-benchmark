//! CWE-362: Constant-folded condition always takes atomic path; non-atomic branch is dead code.

use std::sync::atomic::{AtomicI64, Ordering};

// vuln-code-snippet start testcodeRaceCondition046
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: i64 = req.param("amount").parse().unwrap_or(0);
    static BALANCE: AtomicI64 = AtomicI64::new(1000);
    let result = if 10 * 10 >= 100 {
        BALANCE.fetch_sub(amount, Ordering::SeqCst) // vuln-code-snippet target-line testcodeRaceCondition046
    } else {
        let b = BALANCE.load(Ordering::Relaxed);
        BALANCE.store(b - amount, Ordering::Relaxed);
        b
    };
    super::shared::BenchmarkResponse::ok(&format!("Balance after: {}", result - amount))
}
// vuln-code-snippet end testcodeRaceCondition046
