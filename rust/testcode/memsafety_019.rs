//! CWE-119: mem::transmute between types of different sizes causes undefined behavior.

// vuln-code-snippet start testcodeMemsafety019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: u32 = req.param("value").parse().unwrap_or(0);
    let bytes: [u8; 8] = unsafe { std::mem::transmute(val as u64) }; // vuln-code-snippet target-line testcodeMemsafety019
    super::shared::BenchmarkResponse::ok(&format!("bytes={:?}", &bytes[..]))
}
// vuln-code-snippet end testcodeMemsafety019
