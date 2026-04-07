//! CWE-362: Balance deducted with a single atomic compare-exchange preventing overdraft race.

use std::sync::atomic::{AtomicI64, Ordering};

// vuln-code-snippet start testcodeRaceCondition036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: i64 = req.param("amount").parse().unwrap_or(0);
    static BALANCE: AtomicI64 = AtomicI64::new(1000);
    let prev = BALANCE.fetch_sub(amount, Ordering::SeqCst); // vuln-code-snippet target-line testcodeRaceCondition036
    if prev >= amount {
        super::shared::BenchmarkResponse::ok("Transfer complete")
    } else {
        BALANCE.fetch_add(amount, Ordering::SeqCst);
        super::shared::BenchmarkResponse::bad_request("Insufficient funds")
    }
}
// vuln-code-snippet end testcodeRaceCondition036
