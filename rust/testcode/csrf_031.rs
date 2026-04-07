//! CWE-352: Proper CSRF protection — Origin header validation before password change.

fn change_password(new_password: &str) -> bool {
    let _ = new_password;
    true
}

// vuln-code-snippet start testcodeCsrf031
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if req.header("Origin") != "https://app.example.com" {
        return super::shared::BenchmarkResponse::forbidden("invalid origin");
    }
    let new_password = req.param("new_password");
    let result = change_password(&new_password); // vuln-code-snippet target-line testcodeCsrf031
    if result {
        super::shared::BenchmarkResponse::ok("password changed")
    } else {
        super::shared::BenchmarkResponse::error("change failed")
    }
}
// vuln-code-snippet end testcodeCsrf031
