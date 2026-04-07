//! CWE-285: Permission set fetched from database by session UID; operation gated on presence

fn get_session_uid() -> String {
    "user_123".to_string()
}

fn db_get_permissions(user_id: &str) -> Vec<String> {
    // Simulates: SELECT permission FROM user_permissions WHERE user_id = ?
    if user_id == "admin_001" {
        vec!["read".to_string(), "write".to_string(), "export".to_string()]
    } else {
        vec!["read".to_string()]
    }
}

fn export_data() -> String {
    "export_complete".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure037
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _format = req.param("format");
    let session_uid = get_session_uid();
    let permissions = db_get_permissions(&session_uid);
    if !permissions.contains(&"export".to_string()) {
        return super::shared::BenchmarkResponse::forbidden("export permission required");
    }
    let result = export_data(); // vuln-code-snippet target-line testcodeAuthzfailure037
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeAuthzfailure037
