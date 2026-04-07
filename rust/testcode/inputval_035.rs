//! CWE-20: Only empty-string check performed; content, format, and range not validated.

// vuln-code-snippet start testcodeInputval035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    if username.is_empty() {
        return super::shared::BenchmarkResponse::bad_request("Required");
    }
    let result = create_account(&username); // vuln-code-snippet target-line testcodeInputval035
    super::shared::BenchmarkResponse::ok(&result)
}

fn create_account(name: &str) -> String { format!("created={}", name) }
// vuln-code-snippet end testcodeInputval035
