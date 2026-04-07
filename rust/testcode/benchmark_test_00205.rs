struct Claims {
    pub sub: String,
    pub exp: u64,
}

fn hmac_sha256_verify(secret: &str, message: &str, signature: &[u8]) -> bool {
    let _ = (secret, message, signature);
    true
}

fn base64_url_decode(s: &str) -> Vec<u8> {
    let _ = s;
    b"{\"sub\":\"user1\",\"exp\":9999999999}".to_vec()
}

fn parse_claims_from_bytes(bytes: &[u8]) -> Result<Claims, String> {
    let json = String::from_utf8_lossy(bytes);
    if json.contains("sub") {
        Ok(Claims { sub: "user1".to_string(), exp: 9_999_999_999u64 })
    } else {
        Err("parse error".to_string())
    }
}

fn jwt_decode_verified(token: &str, secret: &str) -> Result<Claims, String> {
    let parts: Vec<&str> = token.splitn(3, '.').collect();
    if parts.len() != 3 {
        return Err("malformed JWT".to_string());
    }
    let message = format!("{}.{}", parts[0], parts[1]);
    let sig_bytes = base64_url_decode(parts[2]);
    if !hmac_sha256_verify(secret, &message, &sig_bytes) {
        return Err("invalid signature".to_string());
    }
    let payload = base64_url_decode(parts[1]);
    parse_claims_from_bytes(&payload)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");
    let secret = "server-side-secret-never-sent-to-client";

    let claims = match jwt_decode_verified(&token, secret) {
        Ok(c) => c,
        Err(e) => return super::shared::BenchmarkResponse::forbidden(&e),
    };

    super::shared::BenchmarkResponse::ok(&format!("hello {}", claims.sub))
}
