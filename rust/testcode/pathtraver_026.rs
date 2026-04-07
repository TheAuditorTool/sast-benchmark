//! CWE-22: User-controlled directory path passed to fs::read_dir(), enabling arbitrary listing.

// vuln-code-snippet start testcodePathtraver026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dir = req.param("dir");

    match std::fs::read_dir(&dir) { // vuln-code-snippet target-line testcodePathtraver026
        Ok(entries) => {
            let names: Vec<String> = entries
                .filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect();
            super::shared::BenchmarkResponse::ok(&names.join("\n"))
        }
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver026
