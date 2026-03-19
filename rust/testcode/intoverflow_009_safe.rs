//! Integer Overflow True Negative — CWE-190
//! Shift amount bounded to less than 32 before shifting. Prevents
//! overflow, panic, and undefined behavior from large shift values.

// vuln-code-snippet start testcodeIntoverflow009Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let base: u32 = req.param("base").parse().unwrap_or(1);
    let shift: u32 = req.param("shift").parse().unwrap_or(0);

    // SAFE: Shift amount bounded to valid range before use
    if shift >= 32 { // vuln-code-snippet safe-line testcodeIntoverflow009Safe
        return super::shared::BenchmarkResponse::bad_request("Shift amount too large");
    }

    let result = base << shift;
    super::shared::BenchmarkResponse::ok(&format!("Shifted: {}", result))
}
// vuln-code-snippet end testcodeIntoverflow009Safe
