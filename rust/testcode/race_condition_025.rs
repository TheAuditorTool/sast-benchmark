//! CWE-362: Directory existence checked before file creation; directory may be replaced between checks.

use std::path::Path;

// vuln-code-snippet start testcodeRaceCondition025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dir = req.param("dir");
    let filename = req.param("file");
    let dirpath = Path::new(&dir);
    if dirpath.is_dir() { // vuln-code-snippet target-line testcodeRaceCondition025
        let full_path = format!("{}/{}", dir, filename);
        let _ = std::fs::write(&full_path, b"data");
        super::shared::BenchmarkResponse::ok("File created")
    } else {
        super::shared::BenchmarkResponse::bad_request("Directory not found")
    }
}
// vuln-code-snippet end testcodeRaceCondition025
