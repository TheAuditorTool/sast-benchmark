//! CWE-190: TryFrom for narrowing cast. Returns Err if value does not fit.

// vuln-code-snippet start testcodeIntoverflow006
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let big_val: u64 = req.param("value").parse().unwrap_or(0);

    match u32::try_from(big_val) { // vuln-code-snippet target-line testcodeIntoverflow006
        Ok(v) => super::shared::BenchmarkResponse::ok(&format!("Value: {}", v)),
        Err(_) => super::shared::BenchmarkResponse::bad_request("Value too large for u32"),
    }
}
// vuln-code-snippet end testcodeIntoverflow006
