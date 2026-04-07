struct SessionUser {
    role: String,
}

impl SessionUser {
    fn has_role(&self, role: &str) -> bool {
        self.role == role
    }
}

fn get_session_user() -> SessionUser {
    SessionUser { role: "user".to_string() }
}

fn delete_all_users() -> String {
    "all_users_deleted".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");
    let session_user = get_session_user();
    if !session_user.has_role("admin") {
        return super::shared::BenchmarkResponse::forbidden("admin role required");
    }
    let result = delete_all_users();
    super::shared::BenchmarkResponse::ok(&result)
}
