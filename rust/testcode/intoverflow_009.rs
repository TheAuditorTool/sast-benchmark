//! CWE-190: Shift amount bounded to less than 32 before shifting.

// vuln-code-snippet start testcodeIntoverflow009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let base: u32 = req.param("base").parse().unwrap_or(1);
    let shift: u32 = req.param("shift").parse().unwrap_or(0);

    if shift >= 32 { // vuln-code-snippet target-line testcodeIntoverflow009
        return super::shared::BenchmarkResponse::bad_request("Shift amount too large");
    }

    let result = base << shift;
    super::shared::BenchmarkResponse::ok(&format!("Shifted: {}", result))
}
// vuln-code-snippet end testcodeIntoverflow009
