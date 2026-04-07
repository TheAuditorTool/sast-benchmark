//! CWE-20: Payment amount validated to be positive before processing.

// vuln-code-snippet start testcodeInputval039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: i64 = req.param("amount").parse().unwrap_or(0);
    if amount <= 0 {
        return super::shared::BenchmarkResponse::bad_request("Amount must be positive");
    }
    let result = process_payment(amount); // vuln-code-snippet target-line testcodeInputval039
    super::shared::BenchmarkResponse::ok(&format!("processed={}", result))
}

fn process_payment(amount: i64) -> i64 { amount }
// vuln-code-snippet end testcodeInputval039
