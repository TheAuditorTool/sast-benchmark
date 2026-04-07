//! CWE-327: JWT decoded with HS256 algorithm and secret key; signature verification enforced.

// vuln-code-snippet start testcodeCrypto035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let secret = get_jwt_secret();
    let claims = decode_jwt_verified(&token, secret); // vuln-code-snippet target-line testcodeCrypto035
    match claims {
        Ok(c) => super::shared::BenchmarkResponse::ok(&c),
        Err(e) => super::shared::BenchmarkResponse::forbidden(&e),
    }
}

fn get_jwt_secret() -> &'static str { "secure-secret-key" }
fn decode_jwt_verified(_token: &str, _secret: &str) -> Result<String, String> {
    Ok("user=verified".to_string())
}
// vuln-code-snippet end testcodeCrypto035
