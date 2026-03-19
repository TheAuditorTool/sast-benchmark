//! Input Validation True Positive — CWE-20
//! Negative payment amount accepted without validation.

// vuln-code-snippet start testcodeInputval001Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount_str = req.param("amount");

    // VULNERABLE: Negative amounts accepted — attacker can credit themselves
    let amount: f64 = amount_str.parse().unwrap_or(0.0); // vuln-code-snippet vuln-line testcodeInputval001Vulnerable
    let balance = 1000.0 - amount;

    super::shared::BenchmarkResponse::ok(&format!("Payment of {} processed. Balance: {}", amount, balance))
}
// vuln-code-snippet end testcodeInputval001Vulnerable
