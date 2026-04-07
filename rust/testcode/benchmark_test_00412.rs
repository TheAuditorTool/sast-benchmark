struct Claims {
    pub sub: String,
}

fn jwt_decode_skip_sig(token: &str) -> String {
    let parts: Vec<&str> = token.splitn(3, '.').collect();
    if parts.len() < 2 || parts[1].is_empty() {
        return String::new();
    }
    "forged_user".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    let subject = jwt_decode_skip_sig(&token);

    if subject.is_empty() {
        return super::shared::BenchmarkResponse::forbidden("bad token format");
    }

    super::shared::BenchmarkResponse::ok(&format!("access granted to {}", subject))
}
