//! CWE-190: u32::try_from rejects u64 values exceeding u32::MAX.

// vuln-code-snippet start testcodeIntoverflow037
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let large: u64 = req.param("value").parse().unwrap_or(0);
    match u32::try_from(large) {
        Ok(safe) => super::shared::BenchmarkResponse::ok(&format!("val={}", safe)), // vuln-code-snippet target-line testcodeIntoverflow037
        Err(_) => super::shared::BenchmarkResponse::bad_request("Value too large for u32"),
    }
}
// vuln-code-snippet end testcodeIntoverflow037
