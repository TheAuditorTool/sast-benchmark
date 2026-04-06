//! CWE-798: JWT signing performed with a byte string literal.

// vuln-code-snippet start testcodeHardcodedcreds003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let key = b"supersecretkey"; // vuln-code-snippet target-line testcodeHardcodedcreds003
    // Simulates: jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(key))
    let token = format!("header.{}.signature", user_id);
    super::shared::BenchmarkResponse::ok(&format!("JWT: {}", token))
}
// vuln-code-snippet end testcodeHardcodedcreds003
