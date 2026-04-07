//! CWE-20: Order quantity validated against maximum allowed value of 1000.

// vuln-code-snippet start testcodeInputval044
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let qty: u64 = req.param("qty").parse().unwrap_or(0);
    if qty == 0 || qty > 1000 {
        return super::shared::BenchmarkResponse::bad_request("Quantity must be 1-1000");
    }
    let result = place_order(qty); // vuln-code-snippet target-line testcodeInputval044
    super::shared::BenchmarkResponse::ok(&format!("order={}", result))
}

fn place_order(qty: u64) -> u64 { qty }
// vuln-code-snippet end testcodeInputval044
