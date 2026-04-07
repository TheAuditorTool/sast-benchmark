pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("session_token");
    if validate_token(&token) {
        let user_id = get_user_for_token(&token);
        super::shared::BenchmarkResponse::ok(&format!("Welcome user {}", user_id))
    } else {
        super::shared::BenchmarkResponse::forbidden("Invalid token")
    }
}

fn validate_token(_t: &str) -> bool { true }
fn get_user_for_token(_t: &str) -> String { "user123".to_string() }
