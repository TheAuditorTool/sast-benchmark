fn session_token_to_uid(token: &str) -> Option<String> {
    if token.starts_with("Bearer ") { Some("user_123".to_string()) } else { None }
}

fn db_get_user_role(uid: &str) -> Option<String> {
    if uid == "admin_001" { Some("admin".to_string()) } else { Some("user".to_string()) }
}

fn privileged_action() -> String {
    "privileged_action_result".to_string()
}

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
    let result = privileged_action();
    super::shared::BenchmarkResponse::ok(&result)
}
