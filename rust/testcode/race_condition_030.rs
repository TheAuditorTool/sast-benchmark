//! CWE-362: Mutex guard dropped after check; re-acquired for write allows concurrent modification between.

use std::sync::{Arc, Mutex};

// vuln-code-snippet start testcodeRaceCondition030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: i64 = req.param("amount").parse().unwrap_or(0);
    let balance_lock = get_balance_lock();
    let current = {
        let guard = balance_lock.lock().unwrap();
        *guard
    };
    if current >= amount { // vuln-code-snippet target-line testcodeRaceCondition030
        let mut guard = balance_lock.lock().unwrap();
        *guard -= amount;
        super::shared::BenchmarkResponse::ok("Done")
    } else {
        super::shared::BenchmarkResponse::bad_request("Low balance")
    }
}

fn get_balance_lock() -> Arc<Mutex<i64>> {
    Arc::new(Mutex::new(1000))
}
// vuln-code-snippet end testcodeRaceCondition030
