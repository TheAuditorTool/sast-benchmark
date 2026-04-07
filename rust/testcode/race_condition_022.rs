//! CWE-362: File metadata checked separately from file read, allowing TOCTOU replacement.

use std::path::Path;

// vuln-code-snippet start testcodeRaceCondition022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filepath = req.param("file");
    let path = Path::new(&filepath);
    let meta = std::fs::metadata(path); // vuln-code-snippet target-line testcodeRaceCondition022
    match meta {
        Ok(m) if m.is_file() => {
            let data = std::fs::read_to_string(path).unwrap_or_default();
            super::shared::BenchmarkResponse::ok(&data)
        }
        _ => super::shared::BenchmarkResponse::bad_request("Not a file"),
    }
}
// vuln-code-snippet end testcodeRaceCondition022
