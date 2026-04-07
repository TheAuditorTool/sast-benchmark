//! CWE-119: User-supplied offset flows through variable to unsafe pointer arithmetic.

// vuln-code-snippet start testcodeMemsafety021
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw_offset: usize = req.param("offset").parse().unwrap_or(0);
    let buf_offset = raw_offset;
    let data = [0u8; 128];
    let val = unsafe { *data.as_ptr().add(buf_offset) }; // vuln-code-snippet target-line testcodeMemsafety021
    super::shared::BenchmarkResponse::ok(&format!("val={}", val))
}
// vuln-code-snippet end testcodeMemsafety021
