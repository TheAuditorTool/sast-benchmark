//! CWE-362: Shared state accessed through Arc<RwLock> with single write guard across check and update.

use std::sync::{Arc, RwLock};

// vuln-code-snippet start testcodeRaceCondition044
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let new_status = req.param("status");
    let state = get_shared_state();
    let mut guard = state.write().unwrap();
    guard.status = new_status; // vuln-code-snippet target-line testcodeRaceCondition044
    super::shared::BenchmarkResponse::ok("Status updated")
}

struct AppState { status: String }
fn get_shared_state() -> Arc<RwLock<AppState>> {
    Arc::new(RwLock::new(AppState { status: "idle".to_string() }))
}
// vuln-code-snippet end testcodeRaceCondition044
