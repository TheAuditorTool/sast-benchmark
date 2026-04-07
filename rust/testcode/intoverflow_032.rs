//! CWE-190: Bounds check uses i32 maximum but value stored as usize; mismatch allows overflow.

// vuln-code-snippet start testcodeIntoverflow032
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: usize = req.param("value").parse().unwrap_or(0);
    if val > i32::MAX as usize {
        return super::shared::BenchmarkResponse::bad_request("Too large");
    }
    let result = val * val; // vuln-code-snippet target-line testcodeIntoverflow032
    super::shared::BenchmarkResponse::ok(&format!("result={}", result))
}
// vuln-code-snippet end testcodeIntoverflow032
