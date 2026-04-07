//! CWE-362: File existence checked separately from read, creating TOCTOU window.

use std::path::Path;

// vuln-code-snippet start testcodeRaceCondition021
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filepath = req.param("path");
    let path = Path::new(&filepath);
    if path.exists() { // vuln-code-snippet target-line testcodeRaceCondition021
        let content = std::fs::read_to_string(path).unwrap_or_default();
        super::shared::BenchmarkResponse::ok(&content)
    } else {
        super::shared::BenchmarkResponse::bad_request("Not found")
    }
}
// vuln-code-snippet end testcodeRaceCondition021
