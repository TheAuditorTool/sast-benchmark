fn reset_password(user_id: &str, new_pwd: &str) -> bool {
    let _ = (user_id, new_pwd);
    true
}

fn get_expected_token() -> String {
    "expected_session_token".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("csrf_token");
    let expected = get_expected_token();
    if token == expected {
        let user_id = req.param("user_id");
        let new_pwd = req.param("new_pwd");
        let result = reset_password(&user_id, &new_pwd);
        if result {
            return super::shared::BenchmarkResponse::ok("password reset");
        }
        return super::shared::BenchmarkResponse::error("reset failed");
    }
    super::shared::BenchmarkResponse::forbidden("csrf mismatch")
}
