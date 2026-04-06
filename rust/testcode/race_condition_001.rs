//! CWE-362: File existence checked before read, allowing state change between check and use.

use std::path::Path;

// vuln-code-snippet start testcodeRaceCondition001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filepath = req.param("path");
    let path = Path::new(&filepath);

    if path.exists() { // vuln-code-snippet target-line testcodeRaceCondition001
        match std::fs::read_to_string(path) {
            Ok(content) => super::shared::BenchmarkResponse::ok(&content),
            Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
        }
    } else {
        super::shared::BenchmarkResponse::bad_request("File not found")
    }
}
// vuln-code-snippet end testcodeRaceCondition001
