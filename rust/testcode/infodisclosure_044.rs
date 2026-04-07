//! CWE-209: Detailed error message unconditionally replaced with generic message before response.

// vuln-code-snippet start testcodeInfodisclosure044
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let mut error_msg = "DB error: table users does not exist".to_string();
    error_msg = "Service temporarily unavailable".to_string();
    let _ = id;
    super::shared::BenchmarkResponse::error(&error_msg) // vuln-code-snippet target-line testcodeInfodisclosure044
}
// vuln-code-snippet end testcodeInfodisclosure044
