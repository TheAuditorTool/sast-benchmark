//! CWE-502: User payload stored as validated UTF-8 string; no object deserialization performed.

// vuln-code-snippet start testcodeDeser046
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let safe_string = validate_utf8(&body); // vuln-code-snippet target-line testcodeDeser046
    super::shared::BenchmarkResponse::ok(&format!("received_len={}", safe_string.len()))
}

fn validate_utf8(s: &str) -> &str {
    s
}
// vuln-code-snippet end testcodeDeser046
