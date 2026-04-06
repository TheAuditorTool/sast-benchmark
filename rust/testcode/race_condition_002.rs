//! CWE-362: Directory creation preceded by non-atomic existence check.

use std::path::Path;

// vuln-code-snippet start testcodeRaceCondition002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dir_name = req.param("dir");
    let path = Path::new(&dir_name);

    if !path.exists() { // vuln-code-snippet target-line testcodeRaceCondition002
        match std::fs::create_dir(path) {
            Ok(_) => super::shared::BenchmarkResponse::ok("Directory created"),
            Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
        }
    } else {
        super::shared::BenchmarkResponse::ok("Directory already exists")
    }
}
// vuln-code-snippet end testcodeRaceCondition002
