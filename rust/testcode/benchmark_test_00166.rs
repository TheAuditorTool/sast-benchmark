struct Session {
    pub username: String,
    pub expires_at: u64,
}

fn db_lookup_session(token: &str) -> Result<Session, String> {
    if token.is_empty() {
        return Err("missing session".to_string());
    }
    Ok(Session { username: "bob".to_string(), expires_at: 9_999_999_999u64 })
}

fn now() -> u64 {
    1_700_000_000u64
}

fn serve_content(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("content for {}", username))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.cookie("session");

    let session = match db_lookup_session(&token) {
        Ok(s) => s,
        Err(e) => return super::shared::BenchmarkResponse::forbidden(&e),
    };

    if session.expires_at < now() {
        return super::shared::BenchmarkResponse::forbidden("session expired");
    }

    serve_content(&session.username)
}
