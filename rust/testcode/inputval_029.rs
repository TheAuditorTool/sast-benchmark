//! CWE-20: Order quantity not validated against maximum allowed value.

// vuln-code-snippet start testcodeInputval029
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let quantity: u64 = req.param("qty").parse().unwrap_or(0);
    let result = place_order(quantity); // vuln-code-snippet target-line testcodeInputval029
    super::shared::BenchmarkResponse::ok(&format!("order={}", result))
}

fn place_order(qty: u64) -> u64 { qty }
// vuln-code-snippet end testcodeInputval029
