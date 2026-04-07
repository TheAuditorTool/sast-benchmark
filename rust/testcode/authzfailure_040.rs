//! CWE-285: JWT verified server-side; uid and role fetched from DB, not request

fn session_token_to_uid(token: &str) -> Option<String> {
    // Simulates JWT signature verification; returns uid from verified claims
    if token.starts_with("Bearer ") { Some("user_123".to_string()) } else { None }
}

fn db_get_user_role(uid: &str) -> Option<String> {
    // Simulates: SELECT role FROM users WHERE id = ?
    if uid == "admin_001" { Some("admin".to_string()) } else { Some("user".to_string()) }
}

fn privileged_action() -> String {
    "privileged_action_result".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure040
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");
    let uid = match session_token_to_uid(&token) {
        Some(u) => u,
        None => return super::shared::BenchmarkResponse::forbidden("invalid token"),
    };
    let role = match db_get_user_role(&uid) {
        Some(r) => r,
        None => return super::shared::BenchmarkResponse::forbidden("user not found"),
    };
    if role != "admin" {
        return super::shared::BenchmarkResponse::forbidden("admin role required");
    }
    let result = privileged_action(); // vuln-code-snippet target-line testcodeAuthzfailure040
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeAuthzfailure040
