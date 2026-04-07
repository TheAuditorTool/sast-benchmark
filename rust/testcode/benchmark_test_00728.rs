const SESSION_UID: &str = "user_123";

fn db_fetch_role(user_id: &str) -> String {
    if user_id == "admin_001" { "admin".to_string() } else { "user".to_string() }
}

fn get_user_role(_req_role_hint: &str) -> String {
    db_fetch_role(SESSION_UID)
}

fn privileged_action() -> String {
    "privileged_action_result".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let role_hint = req.header("X-Role");
    let role = get_user_role(&role_hint);
    if role != "admin" {
        return super::shared::BenchmarkResponse::forbidden("admin role required");
    }
    let result = privileged_action();
    super::shared::BenchmarkResponse::ok(&result)
}
