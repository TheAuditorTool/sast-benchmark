//! CWE-285: Explicit permission check via has_permission before user deletion

fn get_session_uid() -> String {
    "user_123".to_string()
}

fn has_permission(user_id: &str, permission: &str) -> bool {
    // Simulates: SELECT 1 FROM user_permissions WHERE user_id = ? AND permission = ?
    user_id == "admin_001" && permission == "delete_users"
}

fn delete_users() -> String {
    "users_deleted".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _data = req.param("data");
    let session_uid = get_session_uid();
    if !has_permission(&session_uid, "delete_users") {
        return super::shared::BenchmarkResponse::forbidden("permission denied");
    }
    let result = delete_users(); // vuln-code-snippet target-line testcodeAuthzfailure035
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeAuthzfailure035
