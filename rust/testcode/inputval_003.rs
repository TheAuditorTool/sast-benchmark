//! CWE-20: Oversized allocation from user-controlled size parameter.

// vuln-code-snippet start testcodeInputval003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let size_str = req.param("size");
    let size: usize = size_str.parse().unwrap_or(0);

    let buffer: Vec<u8> = vec![0u8; size]; // vuln-code-snippet target-line testcodeInputval003

    super::shared::BenchmarkResponse::ok(&format!("Allocated {} bytes", buffer.len()))
}
// vuln-code-snippet end testcodeInputval003
