fn db_get_token(session_id: &str) -> String {
    let _ = session_id;
    "db_stored_token".to_string()
}

fn update_email(email: &str) -> bool {
    let _ = email;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let session_id = req.cookie("session_id");
    let expected = db_get_token(&session_id);
    if req.param("token") == expected {
        let email = req.param("email");
        let result = update_email(&email);
        if result {
            return super::shared::BenchmarkResponse::ok("email updated");
        }
        return super::shared::BenchmarkResponse::error("update failed");
    }
    super::shared::BenchmarkResponse::forbidden("csrf mismatch")
}
