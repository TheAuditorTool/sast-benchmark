//! CWE-20: Username accepted with special characters that could cause downstream issues.

// vuln-code-snippet start testcodeInputval033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let stored = format!("user:{}", username);
    let result = save_record(&stored); // vuln-code-snippet target-line testcodeInputval033
    super::shared::BenchmarkResponse::ok(&result)
}

fn save_record(r: &str) -> String { format!("saved={}", r) }
// vuln-code-snippet end testcodeInputval033
