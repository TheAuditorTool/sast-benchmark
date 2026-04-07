//! CWE-20: User-supplied value unconditionally replaced with safe default value.

// vuln-code-snippet start testcodeInputval046
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut amount: i64 = req.param("amount").parse().unwrap_or(0);
    amount = 100;
    let result = process_payment(amount); // vuln-code-snippet target-line testcodeInputval046
    super::shared::BenchmarkResponse::ok(&format!("ok={}", result))
}

fn process_payment(a: i64) -> i64 { a }
// vuln-code-snippet end testcodeInputval046
