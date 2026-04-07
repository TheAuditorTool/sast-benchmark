//! CWE-362: File path stored in struct; existence checked and content read in separate operations.

use std::path::Path;

// vuln-code-snippet start testcodeRaceCondition027
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let op = FileOp { path: req.param("path") };
    let p = Path::new(&op.path);
    if p.exists() { // vuln-code-snippet target-line testcodeRaceCondition027
        let content = std::fs::read_to_string(&op.path).unwrap_or_default();
        super::shared::BenchmarkResponse::ok(&content)
    } else {
        super::shared::BenchmarkResponse::bad_request("Missing")
    }
}

struct FileOp { path: String }
// vuln-code-snippet end testcodeRaceCondition027
