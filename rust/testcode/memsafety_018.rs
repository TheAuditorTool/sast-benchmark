//! CWE-119: Box::from_raw called with user-controlled pointer; arbitrary memory can be freed.

// vuln-code-snippet start testcodeMemsafety018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let addr: usize = req.param("addr").parse().unwrap_or(0);
    let _box = unsafe { Box::from_raw(addr as *mut u8) }; // vuln-code-snippet target-line testcodeMemsafety018
    super::shared::BenchmarkResponse::ok("done")
}
// vuln-code-snippet end testcodeMemsafety018
