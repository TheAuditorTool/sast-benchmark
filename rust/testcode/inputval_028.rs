//! CWE-20: Username accepted and processed without checking for empty string.

// vuln-code-snippet start testcodeInputval028
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let result = create_account(&username); // vuln-code-snippet target-line testcodeInputval028
    super::shared::BenchmarkResponse::ok(&result)
}

fn create_account(name: &str) -> String { format!("created={}", name) }
// vuln-code-snippet end testcodeInputval028
