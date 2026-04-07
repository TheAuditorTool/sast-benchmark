fn get_stored_session_token(user_id: &str) -> String {
    let _ = user_id;
    "a3f9b2c1d4e5f6a7b8c9d0e1f2a3b4c5".to_string()
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn grant_access(user_id: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("session valid for {}", user_id))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let session_token = req.cookie("session_token");

    let stored_token = get_stored_session_token(&user_id);

    if constant_time_eq(session_token.as_bytes(), stored_token.as_bytes()) {
        grant_access(&user_id)
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid session")
    }
}
