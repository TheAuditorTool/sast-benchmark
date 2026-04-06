//! CWE-362: File advisory lock acquired before performing read-modify-write.

// vuln-code-snippet start testcodeRaceCondition018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filepath = req.param("path");
    let content = req.body_str();

    // Simulates: fs2::FileExt::lock_exclusive(&file)
    let result = lock_and_write(&filepath, &content); // vuln-code-snippet target-line testcodeRaceCondition018
    super::shared::BenchmarkResponse::ok(&format!("Written with lock: {}", result))
}

fn lock_and_write(path: &str, content: &str) -> String {
    // Simulates: file.lock_exclusive()?; file.write_all(content)?; file.unlock()?;
    format!("locked_write_{}_{}", path, content.len())
}
// vuln-code-snippet end testcodeRaceCondition018
