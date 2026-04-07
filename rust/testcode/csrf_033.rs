//! CWE-352: Proper CSRF protection — Origin header validation before account deletion.

fn delete_account(user_id: &str) -> bool {
    let _ = user_id;
    true
}

// vuln-code-snippet start testcodeCsrf033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if req.header("Origin") != "https://app.example.com" {
        return super::shared::BenchmarkResponse::forbidden("invalid origin");
    }
    let user_id = req.param("user_id");
    let result = delete_account(&user_id); // vuln-code-snippet target-line testcodeCsrf033
    if result {
        super::shared::BenchmarkResponse::ok("account deleted")
    } else {
        super::shared::BenchmarkResponse::error("deletion failed")
    }
}
// vuln-code-snippet end testcodeCsrf033
