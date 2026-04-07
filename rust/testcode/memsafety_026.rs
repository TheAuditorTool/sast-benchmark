//! CWE-119: Slice length derived from user input via intermediate calculation.

// vuln-code-snippet start testcodeMemsafety026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let base: usize = req.param("base").parse().unwrap_or(0);
    let extra: usize = req.param("extra").parse().unwrap_or(0);
    let total_len = base + extra;
    let data = [0u8; 256];
    let slice = unsafe { std::slice::from_raw_parts(data.as_ptr(), total_len) }; // vuln-code-snippet target-line testcodeMemsafety026
    super::shared::BenchmarkResponse::ok(&format!("len={}", slice.len()))
}
// vuln-code-snippet end testcodeMemsafety026
