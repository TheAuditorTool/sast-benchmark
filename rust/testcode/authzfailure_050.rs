//! CWE-285: HashMap holds both request role and DB role; privileged action uses only db_role

fn get_session_uid() -> String {
    "user_123".to_string()
}

fn db_get_role(user_id: &str) -> String {
    // Simulates: SELECT role FROM users WHERE id = ?
    if user_id == "admin_001" { "admin".to_string() } else { "user".to_string() }
}

fn privileged_action() -> String {
    "privileged_action_result".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure050
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let session_uid = get_session_uid();
    let mut m = std::collections::HashMap::new();
    m.insert("request_role", req.header("X-Role"));
    m.insert("db_role", db_get_role(&session_uid));
    let role = m.get("db_role").unwrap();
    if role != "admin" {
        return super::shared::BenchmarkResponse::forbidden("admin role required");
    }
    let result = privileged_action(); // vuln-code-snippet target-line testcodeAuthzfailure050
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeAuthzfailure050
