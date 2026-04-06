//! CWE-190: Checked conversion from u64 to u32 with explicit error on overflow.

// vuln-code-snippet start testcodeIntoverflow017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: u64 = req.param("value").parse().unwrap_or(0);
    match u32::try_from(val) { // vuln-code-snippet target-line testcodeIntoverflow017
        Ok(v) => super::shared::BenchmarkResponse::ok(&format!("Value: {}", v)),
        Err(_) => super::shared::BenchmarkResponse::bad_request("Value too large for u32"),
    }
}
// vuln-code-snippet end testcodeIntoverflow017
