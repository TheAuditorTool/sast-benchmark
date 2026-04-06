//! CWE-362: Async mutex guard held across await point for atomic operation.

// vuln-code-snippet start testcodeRaceCondition017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let item = req.param("item");

    // Simulates: tokio::sync::Mutex
    let result = async_mutex_update(&item); // vuln-code-snippet target-line testcodeRaceCondition017
    super::shared::BenchmarkResponse::ok(&format!("Updated: {}", result))
}

fn async_mutex_update(item: &str) -> String {
    // Simulates: let mut guard = mutex.lock().await; guard.push(item); guard.len()
    format!("locked_and_updated_{}", item)
}
// vuln-code-snippet end testcodeRaceCondition017
