//! CWE-20: Email address validated to contain @ and a domain before storage.

// vuln-code-snippet start testcodeInputval043
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    if !is_valid_email(&email) {
        return super::shared::BenchmarkResponse::bad_request("Invalid email");
    }
    let result = save_email(&email); // vuln-code-snippet target-line testcodeInputval043
    super::shared::BenchmarkResponse::ok(&result)
}

fn is_valid_email(s: &str) -> bool {
    let parts: Vec<&str> = s.split('@').collect();
    parts.len() == 2 && parts[1].contains('.') && !parts[0].is_empty()
}

fn save_email(email: &str) -> String { format!("saved={}", email) }
// vuln-code-snippet end testcodeInputval043
