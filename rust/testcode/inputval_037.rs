//! CWE-20: Age parsed as unsigned u32; negative values rejected at parse boundary.

// vuln-code-snippet start testcodeInputval037
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let age = match req.param("age").parse::<u32>() {
        Ok(a) => a,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid age"),
    };
    let result = register_user(age); // vuln-code-snippet target-line testcodeInputval037
    super::shared::BenchmarkResponse::ok(&result)
}

fn register_user(age: u32) -> String { format!("registered age={}", age) }
// vuln-code-snippet end testcodeInputval037
