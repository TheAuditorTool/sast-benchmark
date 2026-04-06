//! CWE-200: Full filesystem path included in error message returned to client.

// vuln-code-snippet start testcodeInfodisclosure017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("file");
    let full_path = format!("/opt/app/data/uploads/{}", filename);
    match std::fs::read_to_string(&full_path) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(_) => super::shared::BenchmarkResponse::error(
            &format!("File not found: {}", full_path) // vuln-code-snippet target-line testcodeInfodisclosure017
        ),
    }
}
// vuln-code-snippet end testcodeInfodisclosure017
