//! CWE-119: Pointer read at user-controlled offset without bounds check.

// vuln-code-snippet start testcodeMemsafety015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let offset: usize = req.param("offset").parse().unwrap_or(0);
    let data = [0u8; 64];
    let value = unsafe { std::ptr::read(data.as_ptr().add(offset)) }; // vuln-code-snippet target-line testcodeMemsafety015
    super::shared::BenchmarkResponse::ok(&format!("val={}", value))
}
// vuln-code-snippet end testcodeMemsafety015
