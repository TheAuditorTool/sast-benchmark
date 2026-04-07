//! CWE-119: Copy length capped to minimum of user value and buffer size.

// vuln-code-snippet start testcodeMemsafety030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_len: usize = req.param("len").parse().unwrap_or(0);
    let src = [0u8; 64];
    let mut dst = [0u8; 64];
    let safe_len = user_len.min(src.len()).min(dst.len());
    unsafe { std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), safe_len) }; // vuln-code-snippet target-line testcodeMemsafety030
    super::shared::BenchmarkResponse::ok(&format!("copied={}", safe_len))
}
// vuln-code-snippet end testcodeMemsafety030
