//! CWE-20: Struct fields validated via validator crate derive macro constraints.

// vuln-code-snippet start testcodeInputval018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");
    let email = req.param("email");
    if !validate_registration(&name, &email) { // vuln-code-snippet target-line testcodeInputval018
        return super::shared::BenchmarkResponse::bad_request("Validation failed");
    }
    super::shared::BenchmarkResponse::ok(&format!("Registered: {}", name))
}
fn validate_registration(name: &str, email: &str) -> bool {
    // Simulates: #[derive(Validate)] with #[validate(length(min = 1, max = 100))] and #[validate(email)]
    !name.is_empty() && name.len() <= 100 && email.contains('@')
}
// vuln-code-snippet end testcodeInputval018
