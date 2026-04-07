fn get_session_user_id() -> String {
    "user_123".to_string()
}

fn db_fetch_role(user_id: &str) -> String {
    if user_id == "admin_001" { "admin".to_string() } else { "user".to_string() }
}

fn admin_panel() -> String {
    "admin_panel_data".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _hint = req.header("X-Role");
    let session_user_id = get_session_user_id();
    let role = db_fetch_role(&session_user_id);
    if role == "admin" {
        let result = admin_panel();
        return super::shared::BenchmarkResponse::ok(&result);
    }
    super::shared::BenchmarkResponse::forbidden("admin role required")
}
