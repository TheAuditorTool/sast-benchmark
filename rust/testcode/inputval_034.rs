//! CWE-20: Range check is inverted, rejecting valid values and accepting out-of-range ones.

// vuln-code-snippet start testcodeInputval034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let age: i32 = req.param("age").parse().unwrap_or(0);
    if age >= 0 && age <= 150 {
        return super::shared::BenchmarkResponse::bad_request("Value out of range");
    }
    let result = register_user(age); // vuln-code-snippet target-line testcodeInputval034
    super::shared::BenchmarkResponse::ok(&result)
}

fn register_user(age: i32) -> String { format!("registered age={}", age) }
// vuln-code-snippet end testcodeInputval034
