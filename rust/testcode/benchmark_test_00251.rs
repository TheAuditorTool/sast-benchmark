fn verify_and_decode_jwt(token: &str) -> Option<String> {
    if token.starts_with("Bearer valid.") { Some("user_123".to_string()) } else { None }
}

fn db_get_resource_for_user(resource_id: &str, user_id: &str) -> Option<String> {
    if resource_id == "res_1" && user_id == "user_123" {
        Some(format!("resource_content_for_{}", resource_id))
    } else {
        None
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");
    let user_id = match verify_and_decode_jwt(&token) {
        Some(uid) => uid,
        None => return super::shared::BenchmarkResponse::forbidden("invalid or expired token"),
    };
    let resource_id = req.param("resource_id");
    match db_get_resource_for_user(&resource_id, &user_id) {
        Some(content) => super::shared::BenchmarkResponse::ok(&content),
        None => super::shared::BenchmarkResponse::forbidden("not found or access denied"),
    }
}
