//! CWE-190: Saturating multiplication caps at max value instead of wrapping.

// vuln-code-snippet start testcodeIntoverflow018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let width: u32 = req.param("width").parse().unwrap_or(0);
    let height: u32 = req.param("height").parse().unwrap_or(0);
    let pixels = width.saturating_mul(height); // vuln-code-snippet target-line testcodeIntoverflow018
    super::shared::BenchmarkResponse::ok(&format!("Pixels: {}", pixels))
}
// vuln-code-snippet end testcodeIntoverflow018
