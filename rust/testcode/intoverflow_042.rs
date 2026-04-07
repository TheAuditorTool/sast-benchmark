//! CWE-190: i32::try_from rejects i64 values outside i32 range.

// vuln-code-snippet start testcodeIntoverflow042
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: i64 = req.param("value").parse().unwrap_or(0);
    match i32::try_from(val) {
        Ok(safe) => super::shared::BenchmarkResponse::ok(&format!("val={}", safe)), // vuln-code-snippet target-line testcodeIntoverflow042
        Err(_) => super::shared::BenchmarkResponse::bad_request("Value out of i32 range"),
    }
}
// vuln-code-snippet end testcodeIntoverflow042
