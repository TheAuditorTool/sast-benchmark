//! CWE-20: Email address accepted and stored without format validation.

// vuln-code-snippet start testcodeInputval026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    let result = save_email(&email); // vuln-code-snippet target-line testcodeInputval026
    super::shared::BenchmarkResponse::ok(&result)
}

fn save_email(email: &str) -> String { format!("saved={}", email) }
// vuln-code-snippet end testcodeInputval026
