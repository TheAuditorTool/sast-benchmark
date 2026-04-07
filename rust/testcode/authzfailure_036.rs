//! CWE-285: Role fetched from database by session ID, not from request; admin panel gated

fn get_session_user_id() -> String {
    "user_123".to_string()
}

fn db_fetch_role(user_id: &str) -> String {
    // Simulates: SELECT role FROM users WHERE id = ?
    if user_id == "admin_001" { "admin".to_string() } else { "user".to_string() }
}

fn admin_panel() -> String {
    "admin_panel_data".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _hint = req.header("X-Role");
    let session_user_id = get_session_user_id();
    let role = db_fetch_role(&session_user_id);
    if role == "admin" {
        let result = admin_panel(); // vuln-code-snippet target-line testcodeAuthzfailure036
        return super::shared::BenchmarkResponse::ok(&result);
    }
    super::shared::BenchmarkResponse::forbidden("admin role required")
}
// vuln-code-snippet end testcodeAuthzfailure036
