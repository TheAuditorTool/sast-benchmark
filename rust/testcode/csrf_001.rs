//! CWE-352: State-mutating password change with no CSRF token check.

fn change_password(new_password: &str) -> bool {
    let _ = new_password;
    true
}

// vuln-code-snippet start testcodeCsrf001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let new_password = req.param("new_password");
    let result = change_password(&new_password); // vuln-code-snippet target-line testcodeCsrf001
    if result {
        super::shared::BenchmarkResponse::ok("password changed")
    } else {
        super::shared::BenchmarkResponse::error("change failed")
    }
}
// vuln-code-snippet end testcodeCsrf001
