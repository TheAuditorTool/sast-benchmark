//! CWE-362: Concurrent map entry API providing atomic read-modify-write.

// vuln-code-snippet start testcodeRaceCondition020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let key = req.param("key");

    // Simulates: DashMap::entry(key).or_insert(0) += 1
    let count = dashmap_entry_update(&key); // vuln-code-snippet target-line testcodeRaceCondition020
    super::shared::BenchmarkResponse::ok(&format!("Count for {}: {}", key, count))
}

fn dashmap_entry_update(key: &str) -> u64 {
    // Simulates: dashmap.entry(key.to_string()).and_modify(|v| *v += 1).or_insert(1)
    let _ = key;
    1
}
// vuln-code-snippet end testcodeRaceCondition020
