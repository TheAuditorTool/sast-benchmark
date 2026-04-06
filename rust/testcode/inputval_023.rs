//! CWE-20: Content-Length checked against maximum before reading request body.

// vuln-code-snippet start testcodeInputval023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content_length: usize = req.header("content-length").parse().unwrap_or(0);
    if content_length > 5 * 1024 * 1024 { // vuln-code-snippet target-line testcodeInputval023
        return super::shared::BenchmarkResponse::bad_request("Body too large (max 5MB)");
    }
    let body = req.body_str();
    super::shared::BenchmarkResponse::ok(&format!("Received: {} bytes", body.len()))
}
// vuln-code-snippet end testcodeInputval023
