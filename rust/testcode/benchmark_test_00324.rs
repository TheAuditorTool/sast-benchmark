struct UserRecord {
    pub username: String,
    pub role: String,
}

fn session_store_lookup(session_id: &str) -> Result<UserRecord, String> {
    if session_id.is_empty() {
        return Err("missing session".to_string());
    }
    Ok(UserRecord { username: "alice".to_string(), role: "user".to_string() })
}

fn serve_dashboard(user: &UserRecord) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("dashboard for {} ({})", user.username, user.role))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let session_id = req.cookie("session_id");

    let user = match session_store_lookup(&session_id) {
        Ok(u) => u,
        Err(e) => return super::shared::BenchmarkResponse::forbidden(&e),
    };

    serve_dashboard(&user)
}
