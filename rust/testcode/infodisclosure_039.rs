//! CWE-200: User list only returned after successful authentication token verification.

// vuln-code-snippet start testcodeInfodisclosure039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");
    if !is_admin_token(&token) {
        return super::shared::BenchmarkResponse::forbidden("Unauthorized");
    }
    let users = list_all_users(); // vuln-code-snippet target-line testcodeInfodisclosure039
    super::shared::BenchmarkResponse::ok(&users)
}

fn is_admin_token(_token: &str) -> bool { true }
fn list_all_users() -> String { "alice,bob".to_string() }
// vuln-code-snippet end testcodeInfodisclosure039
