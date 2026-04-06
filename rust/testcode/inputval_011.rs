//! CWE-20: Email address stored without format validation.

// vuln-code-snippet start testcodeInputval011
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    let result = store_email(&email); // vuln-code-snippet target-line testcodeInputval011
    super::shared::BenchmarkResponse::ok(&format!("Stored: {}", result))
}
fn store_email(email: &str) -> String { format!("stored_{}", email) }
// vuln-code-snippet end testcodeInputval011
