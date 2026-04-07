//! CWE-119: slice::from_raw_parts called with user-controlled length without bounds validation.

// vuln-code-snippet start testcodeMemsafety014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_len: usize = req.param("len").parse().unwrap_or(0);
    let data = [0u8; 64];
    let slice = unsafe { std::slice::from_raw_parts(data.as_ptr(), user_len) }; // vuln-code-snippet target-line testcodeMemsafety014
    super::shared::BenchmarkResponse::ok(&format!("slice_len={}", slice.len()))
}
// vuln-code-snippet end testcodeMemsafety014
