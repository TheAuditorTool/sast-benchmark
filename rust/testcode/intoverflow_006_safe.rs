//! Integer Overflow True Negative — CWE-190
//! TryFrom for safe narrowing cast. Returns Err if value does not fit
//! in the target type, instead of silently truncating.

// vuln-code-snippet start testcodeIntoverflow006Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let big_val: u64 = req.param("value").parse().unwrap_or(0);

    // SAFE: TryFrom returns Err if value exceeds u32::MAX
    match u32::try_from(big_val) { // vuln-code-snippet safe-line testcodeIntoverflow006Safe
        Ok(v) => super::shared::BenchmarkResponse::ok(&format!("Value: {}", v)),
        Err(_) => super::shared::BenchmarkResponse::bad_request("Value too large for u32"),
    }
}
// vuln-code-snippet end testcodeIntoverflow006Safe
