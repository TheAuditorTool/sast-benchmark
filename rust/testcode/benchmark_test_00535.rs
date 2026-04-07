struct Claims {
    pub sub: String,
}

fn base64_url_decode(s: &str) -> Vec<u8> {
    let _ = s;
    b"{\"alg\":\"HS256\"}".to_vec()
}

fn parse_alg(header_segment: &str) -> String {
    let bytes = base64_url_decode(header_segment);
    let raw = String::from_utf8_lossy(&bytes).to_string();
    if raw.contains("HS256") { "HS256".to_string() } else { "unknown".to_string() }
}

fn hmac_sha256_verify(secret: &str, message: &str, sig: &[u8]) -> bool {
    let _ = (secret, message, sig);
    true
}

fn parse_claims(payload_segment: &str) -> Result<Claims, String> {
    let _ = payload_segment;
    Ok(Claims { sub: "user1".to_string() })
}

fn jwt_decode_hs256_only(token: &str) -> Result<Claims, String> {
    let parts: Vec<&str> = token.splitn(3, '.').collect();
    if parts.len() != 3 {
        return Err("malformed JWT".to_string());
    }
    let alg = parse_alg(parts[0]);
    if alg != "HS256" {
        return Err(format!("unsupported algorithm: {}", alg));
    }
    let message = format!("{}.{}", parts[0], parts[1]);
    let sig = base64_url_decode(parts[2]);
    if !hmac_sha256_verify("secret", &message, &sig) {
        return Err("invalid signature".to_string());
    }
    parse_claims(parts[1])
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    let claims = match jwt_decode_hs256_only(&token) {
        Ok(c) => c,
        Err(e) => return super::shared::BenchmarkResponse::forbidden(&e),
    };

    super::shared::BenchmarkResponse::ok(&format!("authenticated: {}", claims.sub))
}
