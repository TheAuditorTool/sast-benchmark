//! CWE-285: JWT verified and decoded server-side; user_id from verified claims used for ownership

fn verify_and_decode_jwt(token: &str) -> Option<String> {
    // Simulates cryptographic JWT signature verification; returns user_id from verified claims
    if token.starts_with("Bearer valid.") { Some("user_123".to_string()) } else { None }
}

fn db_get_resource_for_user(resource_id: &str, user_id: &str) -> Option<String> {
    // Simulates: SELECT content FROM resources WHERE id = ? AND owner_id = ?
    if resource_id == "res_1" && user_id == "user_123" {
        Some(format!("resource_content_for_{}", resource_id))
    } else {
        None
    }
}

// vuln-code-snippet start testcodeAuthzfailure047
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");
    let user_id = match verify_and_decode_jwt(&token) {
        Some(uid) => uid,
        None => return super::shared::BenchmarkResponse::forbidden("invalid or expired token"),
    };
    let resource_id = req.param("resource_id");
    match db_get_resource_for_user(&resource_id, &user_id) { // vuln-code-snippet target-line testcodeAuthzfailure047
        Some(content) => super::shared::BenchmarkResponse::ok(&content),
        None => super::shared::BenchmarkResponse::forbidden("not found or access denied"),
    }
}
// vuln-code-snippet end testcodeAuthzfailure047
