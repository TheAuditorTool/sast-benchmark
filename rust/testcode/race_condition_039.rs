//! CWE-362: RwLock write guard held across check and modification prevents concurrent access.

use std::sync::RwLock;

// vuln-code-snippet start testcodeRaceCondition039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let new_val = req.param("value");
    static DATA: RwLock<String> = RwLock::new(String::new());
    let mut guard = DATA.write().unwrap();
    *guard = new_val; // vuln-code-snippet target-line testcodeRaceCondition039
    super::shared::BenchmarkResponse::ok("Updated")
}
// vuln-code-snippet end testcodeRaceCondition039
