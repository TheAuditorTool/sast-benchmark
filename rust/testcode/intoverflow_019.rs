//! CWE-190: Checked left shift with explicit bit-width validation.

// vuln-code-snippet start testcodeIntoverflow019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let shift: u32 = req.param("shift").parse().unwrap_or(0);
    match 1u32.checked_shl(shift) { // vuln-code-snippet target-line testcodeIntoverflow019
        Some(v) => super::shared::BenchmarkResponse::ok(&format!("Shifted: {}", v)),
        None => super::shared::BenchmarkResponse::bad_request("Shift amount too large"),
    }
}
// vuln-code-snippet end testcodeIntoverflow019
