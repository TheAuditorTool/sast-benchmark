//! CWE-20: Age validated to be within 1..=150 range before processing.

// vuln-code-snippet start testcodeInputval038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let age: i32 = req.param("age").parse().unwrap_or(-1);
    if !(1..=150).contains(&age) {
        return super::shared::BenchmarkResponse::bad_request("Age must be 1-150");
    }
    let result = register_user(age); // vuln-code-snippet target-line testcodeInputval038
    super::shared::BenchmarkResponse::ok(&result)
}

fn register_user(age: i32) -> String { format!("registered age={}", age) }
// vuln-code-snippet end testcodeInputval038
