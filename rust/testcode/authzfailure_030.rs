//! CWE-285: Authorized resource access with session user ID enforced in helper

fn get_session_user_id() -> String {
    "user_123".to_string()
}

fn get_authorized_resource(id: &str, session_user_id: &str) -> Result<String, String> {
    // Simulates: SELECT content FROM resources WHERE id = ? AND owner_id = ?
    if id == "res_1" && session_user_id == "user_123" {
        Ok(format!("resource_content_for_{}", id))
    } else {
        Err("not found or access denied".to_string())
    }
}

// vuln-code-snippet start testcodeAuthzfailure030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let session_uid = get_session_user_id();
    match get_authorized_resource(&id, &session_uid) { // vuln-code-snippet target-line testcodeAuthzfailure030
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::forbidden(&e),
    }
}
// vuln-code-snippet end testcodeAuthzfailure030
