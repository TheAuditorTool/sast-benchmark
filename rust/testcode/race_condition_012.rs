//! CWE-362: Mutex guard held across entire read-modify-write in single scope.

use std::sync::Mutex;

// vuln-code-snippet start testcodeRaceCondition012
static COUNTER: Mutex<u64> = Mutex::new(0);

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    let mut guard = COUNTER.lock().unwrap(); // vuln-code-snippet target-line testcodeRaceCondition012
    *guard += 1;
    let val = *guard;
    drop(guard);

    super::shared::BenchmarkResponse::ok(&format!("Counter: {}", val))
}
// vuln-code-snippet end testcodeRaceCondition012
