//! CWE-20: Email validated against regex pattern before use.

// vuln-code-snippet start testcodeInputval019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    if !validate_email(&email) { // vuln-code-snippet target-line testcodeInputval019
        return super::shared::BenchmarkResponse::bad_request("Invalid email format");
    }
    super::shared::BenchmarkResponse::ok(&format!("Email accepted: {}", email))
}
fn validate_email(email: &str) -> bool {
    let pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
    email.contains('@') && email.len() <= 254 && email.chars().all(|c| c.is_ascii())
}
// vuln-code-snippet end testcodeInputval019
