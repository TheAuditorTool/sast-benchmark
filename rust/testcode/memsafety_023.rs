//! CWE-119: ptr::copy_nonoverlapping copies user-specified number of bytes without limit check.

// vuln-code-snippet start testcodeMemsafety023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let count: usize = req.param("count").parse().unwrap_or(0);
    let src = [0u8; 64];
    let mut dst = [0u8; 64];
    unsafe { std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), count) }; // vuln-code-snippet target-line testcodeMemsafety023
    super::shared::BenchmarkResponse::ok(&format!("copied={}", count))
}
// vuln-code-snippet end testcodeMemsafety023
