//! CWE-190: wrapping_sub on unsigned integer allowing underflow wrap-around.

// vuln-code-snippet start testcodeIntoverflow011
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: u64 = req.param("amount").parse().unwrap_or(0);
    let cost: u64 = req.param("cost").parse().unwrap_or(0);
    let result = amount.wrapping_sub(cost); // vuln-code-snippet target-line testcodeIntoverflow011
    super::shared::BenchmarkResponse::ok(&format!("Balance: {}", result))
}
// vuln-code-snippet end testcodeIntoverflow011
