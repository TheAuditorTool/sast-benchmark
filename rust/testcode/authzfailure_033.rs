//! CWE-285: Role fetched from database by session user ID before admin action

fn get_session_user_id() -> String {
    "user_123".to_string()
}

fn db_get_role(user_id: &str) -> String {
    // Simulates: SELECT role FROM users WHERE id = ?
    if user_id == "admin_001" { "admin".to_string() } else { "user".to_string() }
}

fn admin_action() -> String {
    "admin_action_executed".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _data = req.param("data");
    let session_user_id = get_session_user_id();
    let role = db_get_role(&session_user_id);
    if role != "admin" {
        return super::shared::BenchmarkResponse::forbidden("admin role required");
    }
    let result = admin_action(); // vuln-code-snippet target-line testcodeAuthzfailure033
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeAuthzfailure033
