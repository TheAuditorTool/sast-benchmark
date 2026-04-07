//! CWE-285: Horizontal privilege escalation — user profile accessible by any user_id param

fn db_get_profile(user_id: &str) -> String {
    format!("profile_data_for_{}", user_id)
}

// vuln-code-snippet start testcodeAuthzfailure021
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let profile = db_get_profile(&user_id); // vuln-code-snippet target-line testcodeAuthzfailure021
    super::shared::BenchmarkResponse::ok(&profile)
}
// vuln-code-snippet end testcodeAuthzfailure021
