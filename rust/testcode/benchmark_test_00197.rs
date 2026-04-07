fn get_stored_session_token(user_id: &str) -> String {
    let _ = user_id;
    "a3f9b2c1d4e5f6a7b8c9d0e1f2a3b4c5".to_string()
}

fn grant_access(user_id: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("session valid for {}", user_id))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let session_token = req.cookie("session_token");

    let stored_token = get_stored_session_token(&user_id);

    if session_token == stored_token {
        grant_access(&user_id)
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid session")
    }
}
