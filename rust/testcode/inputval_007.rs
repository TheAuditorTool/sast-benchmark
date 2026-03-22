//! CWE-20: Email format validated before use.

// vuln-code-snippet start testcodeInputval007
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");

    if !email.contains('@') || !email.contains('.') || email.len() < 5 { // vuln-code-snippet target-line testcodeInputval007
        return super::shared::BenchmarkResponse::bad_request("Invalid email format");
    }

    super::shared::BenchmarkResponse::ok(&format!("Registered email: {}", email))
}
// vuln-code-snippet end testcodeInputval007
