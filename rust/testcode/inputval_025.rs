//! CWE-20: Age parameter accepted without range validation allowing negative or unreasonably large values.

// vuln-code-snippet start testcodeInputval025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let age: i32 = req.param("age").parse().unwrap_or(0);
    let result = register_user(age); // vuln-code-snippet target-line testcodeInputval025
    super::shared::BenchmarkResponse::ok(&result)
}

fn register_user(age: i32) -> String { format!("registered age={}", age) }
// vuln-code-snippet end testcodeInputval025
