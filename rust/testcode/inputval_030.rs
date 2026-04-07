//! CWE-20: Float price parsed without checking for NaN or Infinity values.

// vuln-code-snippet start testcodeInputval030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let price: f64 = req.param("price").parse().unwrap_or(0.0);
    let result = calculate_total(price); // vuln-code-snippet target-line testcodeInputval030
    super::shared::BenchmarkResponse::ok(&format!("total={}", result))
}

fn calculate_total(p: f64) -> f64 { p * 1.1 }
// vuln-code-snippet end testcodeInputval030
