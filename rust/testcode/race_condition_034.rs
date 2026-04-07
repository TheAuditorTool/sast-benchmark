//! CWE-362: Balance checked and deducted without holding a lock, allowing concurrent overdraft.

use std::sync::Mutex;

// vuln-code-snippet start testcodeRaceCondition034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: i64 = req.param("amount").parse().unwrap_or(0);
    let balance = read_balance_unsafe();
    if balance >= amount {
        deduct_balance_unsafe(amount); // vuln-code-snippet target-line testcodeRaceCondition034
        super::shared::BenchmarkResponse::ok("Deducted")
    } else {
        super::shared::BenchmarkResponse::bad_request("Insufficient")
    }
}

static MTX: Mutex<i64> = Mutex::new(1000);

fn read_balance_unsafe() -> i64 {
    *MTX.lock().unwrap()
}

fn deduct_balance_unsafe(amt: i64) {
    let mut g = MTX.lock().unwrap();
    *g -= amt;
}
// vuln-code-snippet end testcodeRaceCondition034
