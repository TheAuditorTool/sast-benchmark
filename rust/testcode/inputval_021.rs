//! CWE-20: Numeric input clamped to valid range before arithmetic.

// vuln-code-snippet start testcodeInputval021
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw: i64 = req.param("quantity").parse().unwrap_or(0);
    let quantity = raw.clamp(1, 100); // vuln-code-snippet target-line testcodeInputval021
    let total = quantity * 25;
    super::shared::BenchmarkResponse::ok(&format!("Total: {}", total))
}
// vuln-code-snippet end testcodeInputval021
