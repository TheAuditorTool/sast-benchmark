//! CWE-119: get_unchecked called with user-supplied index without bounds validation.

// vuln-code-snippet start testcodeMemsafety016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let idx: usize = req.param("index").parse().unwrap_or(0);
    let data = [1u8, 2, 3, 4, 5, 6, 7, 8];
    let value = unsafe { *data.get_unchecked(idx) }; // vuln-code-snippet target-line testcodeMemsafety016
    super::shared::BenchmarkResponse::ok(&format!("val={}", value))
}
// vuln-code-snippet end testcodeMemsafety016
