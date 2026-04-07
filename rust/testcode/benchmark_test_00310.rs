fn get_session_uid() -> String {
    "user_123".to_string()
}

fn has_permission(user_id: &str, permission: &str) -> bool {
    user_id == "admin_001" && permission == "delete_users"
}

fn delete_users() -> String {
    "users_deleted".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _data = req.param("data");
    let session_uid = get_session_uid();
    if !has_permission(&session_uid, "delete_users") {
        return super::shared::BenchmarkResponse::forbidden("permission denied");
    }
    let result = delete_users();
    super::shared::BenchmarkResponse::ok(&result)
}
