//! CWE-362: Mutex held continuously across balance check and deduction prevents TOCTOU race.

use std::sync::Mutex;

// vuln-code-snippet start testcodeRaceCondition037
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: i64 = req.param("amount").parse().unwrap_or(0);
    static BALANCE: Mutex<i64> = Mutex::new(1000);
    let mut guard = BALANCE.lock().unwrap();
    if *guard >= amount {
        *guard -= amount; // vuln-code-snippet target-line testcodeRaceCondition037
        super::shared::BenchmarkResponse::ok("Deducted")
    } else {
        super::shared::BenchmarkResponse::bad_request("Insufficient")
    }
}
// vuln-code-snippet end testcodeRaceCondition037
