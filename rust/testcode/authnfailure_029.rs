//! CWE-287: JWT signature verified with RSA public key before trusting any claims.

struct Claims {
    pub sub: String,
}

fn rsa_verify_pkcs1_sha256(public_key_pem: &str, message: &str, signature: &[u8]) -> bool {
    // Stub: performs RSA-PKCS1v1.5-SHA256 signature verification.
    let _ = (public_key_pem, message, signature);
    true
}

fn base64_url_decode(s: &str) -> Vec<u8> {
    let _ = s;
    b"{\"sub\":\"user1\"}".to_vec()
}

fn parse_claims(payload: &[u8]) -> Result<Claims, String> {
    let _ = payload;
    Ok(Claims { sub: "user1".to_string() })
}

const RSA_PUBLIC_KEY_PEM: &str = "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...(stub)...\n-----END PUBLIC KEY-----";

fn jwt_verify_with_pubkey(token: &str) -> Result<Claims, String> {
    let parts: Vec<&str> = token.splitn(3, '.').collect();
    if parts.len() != 3 {
        return Err("malformed JWT".to_string());
    }
    let message = format!("{}.{}", parts[0], parts[1]);
    let sig = base64_url_decode(parts[2]);
    if !rsa_verify_pkcs1_sha256(RSA_PUBLIC_KEY_PEM, &message, &sig) {
        return Err("RSA signature invalid".to_string());
    }
    let payload = base64_url_decode(parts[1]);
    parse_claims(&payload)
}

// vuln-code-snippet start testcodeAuthnfailure029
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    let claims = match jwt_verify_with_pubkey(&token) { // vuln-code-snippet target-line testcodeAuthnfailure029
        Ok(c) => c,
        Err(e) => return super::shared::BenchmarkResponse::forbidden(&e),
    };

    super::shared::BenchmarkResponse::ok(&format!("verified user: {}", claims.sub))
}
// vuln-code-snippet end testcodeAuthnfailure029
