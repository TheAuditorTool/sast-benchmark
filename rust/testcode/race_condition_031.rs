//! CWE-362: Counter read and increment are separate non-atomic operations.

// vuln-code-snippet start testcodeRaceCondition031
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let key = req.param("key");
    let count = read_count(&key); // vuln-code-snippet target-line testcodeRaceCondition031
    write_count(&key, count + 1);
    super::shared::BenchmarkResponse::ok(&format!("Count: {}", count + 1))
}

fn read_count(_key: &str) -> u64 { 0 }
fn write_count(_key: &str, _val: u64) {}
// vuln-code-snippet end testcodeRaceCondition031
