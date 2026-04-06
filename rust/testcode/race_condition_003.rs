//! CWE-362: Mutex lock released between reading and writing shared counter.

use std::sync::Mutex;

// vuln-code-snippet start testcodeRaceCondition003
static COUNTER: Mutex<u64> = Mutex::new(0);

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    let val = *COUNTER.lock().unwrap();
    // Lock dropped here; another thread can modify COUNTER before next lock
    *COUNTER.lock().unwrap() = val + 1; // vuln-code-snippet target-line testcodeRaceCondition003

    super::shared::BenchmarkResponse::ok(&format!("Counter: {}", val + 1))
}
// vuln-code-snippet end testcodeRaceCondition003
