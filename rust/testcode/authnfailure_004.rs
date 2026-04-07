//! CWE-287: JWT expiry (exp) field not checked — expired tokens accepted as valid.

struct Claims {
    pub sub: String,
    pub exp: u64,
}

fn jwt_decode_no_expiry(token: &str) -> String {
    // Stub: decodes and verifies the HMAC signature (simulated) but never checks exp.
    // An attacker can reuse a token indefinitely after it should have expired.
    let _ = token;
    "user_from_expired_token".to_string()
}

fn get_current_timestamp() -> u64 {
    // Stub: returns current unix timestamp.
    1_700_000_000u64
}

// vuln-code-snippet start testcodeAuthnfailure004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    let subject = jwt_decode_no_expiry(&token); // vuln-code-snippet target-line testcodeAuthnfailure004

    if subject.is_empty() {
        return super::shared::BenchmarkResponse::forbidden("invalid token");
    }

    // exp never validated; token accepted even when long past expiry.
    let _ = get_current_timestamp();

    super::shared::BenchmarkResponse::ok(&format!("hello {}", subject))
}
// vuln-code-snippet end testcodeAuthnfailure004
