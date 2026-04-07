//! CWE-200: Full server-side file path exposed to client in error message.

// vuln-code-snippet start testcodeInfodisclosure025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("file");
    let path = format!("/var/app/data/{}", filename);
    match std::fs::read_to_string(&path) {
        Ok(c) => super::shared::BenchmarkResponse::ok(&c),
        Err(e) => super::shared::BenchmarkResponse::error(&format!("Failed to read {}: {}", path, e)), // vuln-code-snippet target-line testcodeInfodisclosure025
    }
}
// vuln-code-snippet end testcodeInfodisclosure025
