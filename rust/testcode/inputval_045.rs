//! CWE-20: Constant-folded condition always applies validation check.

// vuln-code-snippet start testcodeInputval045
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount: i64 = req.param("amount").parse().unwrap_or(0);
    if 1 > 0 {
        if amount <= 0 {
            return super::shared::BenchmarkResponse::bad_request("Must be positive");
        }
        let result = process_payment(amount); // vuln-code-snippet target-line testcodeInputval045
        super::shared::BenchmarkResponse::ok(&format!("ok={}", result))
    } else {
        let r = process_payment(amount);
        super::shared::BenchmarkResponse::ok(&format!("ok={}", r))
    }
}

fn process_payment(a: i64) -> i64 { a }
// vuln-code-snippet end testcodeInputval045
