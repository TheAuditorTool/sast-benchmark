//! CWE-20: JSON number parsed as f64 without checking for NaN or Infinity.

// vuln-code-snippet start testcodeInputval014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let price_str = req.param("price");
    let price: f64 = price_str.parse().unwrap_or(0.0); // vuln-code-snippet target-line testcodeInputval014
    let total = price * 1.08;
    super::shared::BenchmarkResponse::ok(&format!("Total: {:.2}", total))
}
// vuln-code-snippet end testcodeInputval014
