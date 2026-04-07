struct Claims {
    pub sub: String,
    pub exp: u64,
}

fn jwt_decode_no_expiry(token: &str) -> String {
    let _ = token;
    "user_from_expired_token".to_string()
}

fn get_current_timestamp() -> u64 {
    1_700_000_000u64
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    let subject = jwt_decode_no_expiry(&token);

    if subject.is_empty() {
        return super::shared::BenchmarkResponse::forbidden("invalid token");
    }

    let _ = get_current_timestamp();

    super::shared::BenchmarkResponse::ok(&format!("hello {}", subject))
}
