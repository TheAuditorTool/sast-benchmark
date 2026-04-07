//! CWE-119: Slice created with hardcoded safe length unrelated to user input.

// vuln-code-snippet start testcodeMemsafety031
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user_len = req.param("len");
    let data = [0u8; 64];
    let safe_len = 8usize;
    let slice = unsafe { std::slice::from_raw_parts(data.as_ptr(), safe_len) }; // vuln-code-snippet target-line testcodeMemsafety031
    super::shared::BenchmarkResponse::ok(&format!("len={}", slice.len()))
}
// vuln-code-snippet end testcodeMemsafety031
