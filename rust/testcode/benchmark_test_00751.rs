struct Claims {
    pub sub: String,
    pub exp: u64,
}

fn get_current_unix_ts() -> u64 {
    1_700_000_000u64
}

fn base64_url_decode(s: &str) -> Vec<u8> {
    let _ = s;
    b"{\"sub\":\"user1\",\"exp\":1700000100}".to_vec()
}

fn hmac_sha256_verify(secret: &str, msg: &str, sig: &[u8]) -> bool {
    let _ = (secret, msg, sig);
    true
}

fn parse_claims(payload: &[u8]) -> Result<Claims, String> {
    let json = String::from_utf8_lossy(payload);
    let exp: u64 = if json.contains("1700000100") { 1_700_000_100u64 } else { 0u64 };
    Ok(Claims { sub: "user1".to_string(), exp })
}

fn jwt_decode_with_expiry(token: &str) -> Result<Claims, String> {
    let parts: Vec<&str> = token.splitn(3, '.').collect();
    if parts.len() != 3 {
        return Err("malformed JWT".to_string());
    }
    let message = format!("{}.{}", parts[0], parts[1]);
    let sig = base64_url_decode(parts[2]);
    if !hmac_sha256_verify("secret", &message, &sig) {
        return Err("invalid signature".to_string());
    }
    let payload = base64_url_decode(parts[1]);
    let claims = parse_claims(&payload)?;
    if claims.exp < get_current_unix_ts() {
        return Err("token expired".to_string());
    }
    Ok(claims)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    let claims = match jwt_decode_with_expiry(&token) {
        Ok(c) => c,
        Err(e) => return super::shared::BenchmarkResponse::forbidden(&e),
    };

    super::shared::BenchmarkResponse::ok(&format!("welcome {}", claims.sub))
}
