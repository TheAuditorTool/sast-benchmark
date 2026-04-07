//! CWE-119: User-supplied length unconditionally replaced with safe constant before unsafe use.

// vuln-code-snippet start testcodeMemsafety035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut user_len: usize = req.param("len").parse().unwrap_or(0);
    user_len = 8;
    let data = [0u8; 64];
    let slice = unsafe { std::slice::from_raw_parts(data.as_ptr(), user_len) }; // vuln-code-snippet target-line testcodeMemsafety035
    super::shared::BenchmarkResponse::ok(&format!("len={}", slice.len()))
}
// vuln-code-snippet end testcodeMemsafety035
