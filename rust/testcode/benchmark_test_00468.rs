struct AuthContext {
    user_id: String,
    role: String,
}

fn build_auth_context_from_session(session_token: &str) -> AuthContext {
    let _ = session_token;
    AuthContext {
        user_id: "user_123".to_string(),
        role: "user".to_string(),
    }
}

fn privileged_action() -> String {
    "privileged_action_executed".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let session_token = req.header("Authorization");
    let ctx = build_auth_context_from_session(&session_token);
    if ctx.role != "admin" {
        return super::shared::BenchmarkResponse::forbidden("admin role required");
    }
    let result = privileged_action();
    super::shared::BenchmarkResponse::ok(&result)
}
