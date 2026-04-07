//! CWE-20: Payment amount accepted without checking for negative values.

// vuln-code-snippet start testcodeInputval024
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: i64 = req.param("amount").parse().unwrap_or(0);
    let result = process_payment(amount); // vuln-code-snippet target-line testcodeInputval024
    super::shared::BenchmarkResponse::ok(&format!("processed={}", result))
}

fn process_payment(amount: i64) -> i64 { amount }
// vuln-code-snippet end testcodeInputval024
