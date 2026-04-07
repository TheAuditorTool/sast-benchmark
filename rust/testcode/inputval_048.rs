//! CWE-20: User-supplied amount stored under one HashMap key; safe validated amount used for processing.

use std::collections::HashMap;

// vuln-code-snippet start testcodeInputval048
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut amounts = HashMap::new();
    amounts.insert("user_amount", req.param("amount").parse::<i64>().unwrap_or(0));
    amounts.insert("safe_amount", 100i64);
    let amount = amounts.get("safe_amount").unwrap();
    let result = process_payment(*amount); // vuln-code-snippet target-line testcodeInputval048
    super::shared::BenchmarkResponse::ok(&format!("ok={}", result))
}

fn process_payment(a: i64) -> i64 { a }
// vuln-code-snippet end testcodeInputval048
