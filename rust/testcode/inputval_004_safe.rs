//! Input Validation True Negative — CWE-20
//! Payment amount validated with range check before use.

// vuln-code-snippet start testcodeInputval004Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount_str = req.param("amount");
    let amount: f64 = match amount_str.parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid amount"),
    };

    // SAFE: Range validation rejects negative and excessive amounts
    if amount <= 0.0 || amount >= 100_000.0 { // vuln-code-snippet safe-line testcodeInputval004Safe
        return super::shared::BenchmarkResponse::bad_request("Amount must be between 0 and 100000");
    }

    let balance = 1000.0 - amount;
    super::shared::BenchmarkResponse::ok(&format!("Payment of {} processed. Balance: {}", amount, balance))
}
// vuln-code-snippet end testcodeInputval004Safe
