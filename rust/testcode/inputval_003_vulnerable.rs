//! Input Validation True Positive — CWE-20
//! Oversized allocation from user-controlled size parameter.

// vuln-code-snippet start testcodeInputval003Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let size_str = req.param("size");
    let size: usize = size_str.parse().unwrap_or(0);

    // VULNERABLE: User controls allocation size — can request gigabytes, causing OOM
    let buffer: Vec<u8> = vec![0u8; size]; // vuln-code-snippet vuln-line testcodeInputval003Vulnerable

    super::shared::BenchmarkResponse::ok(&format!("Allocated {} bytes", buffer.len()))
}
// vuln-code-snippet end testcodeInputval003Vulnerable
