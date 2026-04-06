//! CWE-20: Request body read without Content-Length check allowing unbounded input.

// vuln-code-snippet start testcodeInputval015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.body_str(); // vuln-code-snippet target-line testcodeInputval015
    super::shared::BenchmarkResponse::ok(&format!("Received {} bytes", body.len()))
}
// vuln-code-snippet end testcodeInputval015
