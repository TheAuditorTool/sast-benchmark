//! CWE-362: File renamed to temp name then read; attacker can swap symlink in the window.

use std::path::Path;

// vuln-code-snippet start testcodeRaceCondition033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let src = req.param("src");
    let tmp = format!("{}.tmp", src);
    if Path::new(&src).exists() {
        let _ = std::fs::rename(&src, &tmp);
        let content = std::fs::read_to_string(&tmp); // vuln-code-snippet target-line testcodeRaceCondition033
        match content {
            Ok(data) => super::shared::BenchmarkResponse::ok(&data),
            Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
        }
    } else {
        super::shared::BenchmarkResponse::bad_request("No such file")
    }
}
// vuln-code-snippet end testcodeRaceCondition033
