//! CWE-362: Entry API provides atomic check-and-insert preventing duplicate insertion race.

// vuln-code-snippet start testcodeRaceCondition041
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let key = req.param("key");
    let value = req.param("value");
    let result = atomic_insert_if_absent(&key, &value); // vuln-code-snippet target-line testcodeRaceCondition041
    if result {
        super::shared::BenchmarkResponse::ok("Inserted")
    } else {
        super::shared::BenchmarkResponse::bad_request("Already exists")
    }
}

fn atomic_insert_if_absent(_key: &str, _val: &str) -> bool {
    // Simulates: dashmap.entry(key).or_insert(val); true
    true
}
// vuln-code-snippet end testcodeRaceCondition041
