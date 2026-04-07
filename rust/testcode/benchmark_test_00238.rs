struct Session {
    pub user_id: u64,
    pub username: String,
}

fn db_lookup_session(token: &str) -> Result<Session, String> {
    if token.is_empty() {
        return Err("missing token".to_string());
    }
    Ok(Session { user_id: 42, username: "alice".to_string() })
}

fn serve_protected_resource(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("resource for {}", username))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.cookie("session_id");

    let session = match db_lookup_session(&token) {
        Ok(s) => s,
        Err(e) => return super::shared::BenchmarkResponse::forbidden(&e),
    };

    serve_protected_resource(&session.username)
}
