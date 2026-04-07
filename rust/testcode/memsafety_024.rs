//! CWE-119: Bounds check uses capacity instead of initialized length; off-by-one still possible.

// vuln-code-snippet start testcodeMemsafety024
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let idx: usize = req.param("index").parse().unwrap_or(0);
    let data = vec![1u8, 2, 3, 4];
    if idx > data.capacity() {
        return super::shared::BenchmarkResponse::bad_request("Out of bounds");
    }
    let val = unsafe { *data.get_unchecked(idx) }; // vuln-code-snippet target-line testcodeMemsafety024
    super::shared::BenchmarkResponse::ok(&format!("val={}", val))
}
// vuln-code-snippet end testcodeMemsafety024
