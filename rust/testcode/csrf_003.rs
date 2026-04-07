//! CWE-352: Account deletion with no CSRF token check.

fn delete_account(user_id: &str) -> bool {
    let _ = user_id;
    true
}

// vuln-code-snippet start testcodeCsrf003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let result = delete_account(&user_id); // vuln-code-snippet target-line testcodeCsrf003
    if result {
        super::shared::BenchmarkResponse::ok("account deleted")
    } else {
        super::shared::BenchmarkResponse::error("deletion failed")
    }
}
// vuln-code-snippet end testcodeCsrf003
