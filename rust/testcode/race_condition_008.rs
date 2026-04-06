//! CWE-362: Lock file checked for existence then assumed to grant exclusive access.

use std::path::Path;

// vuln-code-snippet start testcodeRaceCondition008
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let resource = req.param("resource");
    let lock_path = format!("/tmp/{}.lock", resource);

    if Path::new(&lock_path).exists() { // vuln-code-snippet target-line testcodeRaceCondition008
        return super::shared::BenchmarkResponse::error("Resource locked");
    }

    let _ = std::fs::write(&lock_path, "locked");
    let result = format!("Processing resource: {}", resource);
    let _ = std::fs::remove_file(&lock_path);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeRaceCondition008
