fn get_valid_reset_token(email: &str) -> String {
    let _ = email;
    "reset-token-xyz789abc".to_string()
}

fn constant_time_match(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn complete_password_reset(email: &str, new_password: &str) -> super::shared::BenchmarkResponse {
    let _ = new_password;
    super::shared::BenchmarkResponse::ok(&format!("password reset for {}", email))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    let reset_token = req.param("token");
    let new_password = req.param("new_password");

    let valid_token = get_valid_reset_token(&email);

    if constant_time_match(reset_token.as_bytes(), valid_token.as_bytes()) {
        complete_password_reset(&email, &new_password)
    } else {
        super::shared::BenchmarkResponse::forbidden("invalid or expired reset token")
    }
}
