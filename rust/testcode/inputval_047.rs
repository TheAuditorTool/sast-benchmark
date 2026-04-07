//! CWE-20: Processing helper discards user-supplied value and uses safe internal default.

// vuln-code-snippet start testcodeInputval047
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount = req.param("amount");
    let result = safe_process(&amount); // vuln-code-snippet target-line testcodeInputval047
    super::shared::BenchmarkResponse::ok(&format!("result={}", result))
}

fn safe_process(_user_amount: &str) -> i64 { 100 }
// vuln-code-snippet end testcodeInputval047
