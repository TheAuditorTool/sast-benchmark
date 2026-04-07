//! CWE-285: Admin password reset performed without authorization check

fn reset_user_password(user_id: &str) -> String {
    format!("password_reset_for_{}", user_id)
}

// vuln-code-snippet start testcodeAuthzfailure008
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let result = reset_user_password(&user_id); // vuln-code-snippet target-line testcodeAuthzfailure008
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeAuthzfailure008
