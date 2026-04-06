//! CWE-362: RwLock write guard held for entire read-modify-write transaction.

use std::sync::RwLock;

// vuln-code-snippet start testcodeRaceCondition015
static STATE: RwLock<Vec<String>> = RwLock::new(Vec::new());

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let item = req.param("item");

    let mut guard = STATE.write().unwrap(); // vuln-code-snippet target-line testcodeRaceCondition015
    guard.push(item);
    let count = guard.len();
    drop(guard);

    super::shared::BenchmarkResponse::ok(&format!("Items: {}", count))
}
// vuln-code-snippet end testcodeRaceCondition015
