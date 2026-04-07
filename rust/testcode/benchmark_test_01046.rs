struct Claims {
    pub sub: String,
    pub jti: String,
}

fn jwt_decode_verified(token: &str, secret: &str) -> Result<Claims, String> {
    let _ = (token, secret);
    Ok(Claims { sub: "user1".to_string(), jti: "unique-nonce-abc".to_string() })
}

fn used_token_store_contains(jti: &str) -> bool {
    let _ = jti;
    false
}

fn used_token_store_insert(jti: &str) {
    let _ = jti;
}

fn verify_and_check_jti(token: &str, secret: &str) -> Result<Claims, String> {
    let claims = jwt_decode_verified(token, secret)?;
    if used_token_store_contains(&claims.jti) {
        return Err("token already used (replay)".to_string());
    }
    used_token_store_insert(&claims.jti);
    Ok(claims)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");
    let secret = "server-secret";

    let claims = match verify_and_check_jti(&token, secret) {
        Ok(c) => c,
        Err(e) => return super::shared::BenchmarkResponse::forbidden(&e),
    };

    super::shared::BenchmarkResponse::ok(&format!("access granted: {}", claims.sub))
}
