//! CWE-285: Admin account created without authorization check

fn create_admin_account(username: &str) -> String {
    format!("admin_account_created_for_{}", username)
}

// vuln-code-snippet start testcodeAuthzfailure009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let result = create_admin_account(&username); // vuln-code-snippet target-line testcodeAuthzfailure009
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeAuthzfailure009
