//! CWE-119: ptr::write called with address derived from user-controlled value.

// vuln-code-snippet start testcodeMemsafety022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let addr: usize = req.param("addr").parse().unwrap_or(0);
    let value: u8 = req.param("value").parse().unwrap_or(0);
    unsafe { std::ptr::write(addr as *mut u8, value) }; // vuln-code-snippet target-line testcodeMemsafety022
    super::shared::BenchmarkResponse::ok("written")
}
// vuln-code-snippet end testcodeMemsafety022
