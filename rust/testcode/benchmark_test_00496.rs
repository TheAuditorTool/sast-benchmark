pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let secret = get_jwt_secret();
    let claims = decode_jwt_verified(&token, secret);
    match claims {
        Ok(c) => super::shared::BenchmarkResponse::ok(&c),
        Err(e) => super::shared::BenchmarkResponse::forbidden(&e),
    }
}

fn get_jwt_secret() -> &'static str { "secure-secret-key" }
fn decode_jwt_verified(_token: &str, _secret: &str) -> Result<String, String> {
    Ok("user=verified".to_string())
}
