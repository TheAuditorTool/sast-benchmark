//! CWE-362: Directory listing used to iterate files that may change before operation.

// vuln-code-snippet start testcodeRaceCondition009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dir = req.param("dir");

    let entries = std::fs::read_dir(&dir); // vuln-code-snippet target-line testcodeRaceCondition009
    match entries {
        Ok(listing) => {
            let mut count = 0;
            for entry in listing.flatten() {
                let _ = std::fs::remove_file(entry.path());
                count += 1;
            }
            super::shared::BenchmarkResponse::ok(&format!("Removed {} files", count))
        }
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodeRaceCondition009
