//! CWE-287: JWT jti (nonce) checked against used-token store to prevent replay attacks.

struct Claims {
    pub sub: String,
    pub jti: String,
}

fn jwt_decode_verified(token: &str, secret: &str) -> Result<Claims, String> {
    // Stub: verifies HMAC signature and parses claims including jti.
    let _ = (token, secret);
    Ok(Claims { sub: "user1".to_string(), jti: "unique-nonce-abc".to_string() })
}

fn used_token_store_contains(jti: &str) -> bool {
    // Stub: checks if this jti has been seen before (replay detection).
    let _ = jti;
    false
}

fn used_token_store_insert(jti: &str) {
    // Stub: records jti as consumed so it cannot be reused.
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

// vuln-code-snippet start testcodeAuthnfailure030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");
    let secret = "server-secret";

    let claims = match verify_and_check_jti(&token, secret) { // vuln-code-snippet target-line testcodeAuthnfailure030
        Ok(c) => c,
        Err(e) => return super::shared::BenchmarkResponse::forbidden(&e),
    };

    super::shared::BenchmarkResponse::ok(&format!("access granted: {}", claims.sub))
}
// vuln-code-snippet end testcodeAuthnfailure030
