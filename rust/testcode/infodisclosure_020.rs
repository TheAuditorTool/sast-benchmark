//! CWE-200: Error response contains only a request ID for correlation, no internal details.

// vuln-code-snippet start testcodeInfodisclosure020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");
    let request_id = "req-a1b2c3d4";
    let response = format!(r#"{{"error":"internal","request_id":"{}"}}"#, request_id); // vuln-code-snippet target-line testcodeInfodisclosure020
    super::shared::BenchmarkResponse::error(&response)
}
// vuln-code-snippet end testcodeInfodisclosure020
