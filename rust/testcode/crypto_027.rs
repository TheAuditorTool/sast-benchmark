//! CWE-347: JWT decoded with alg=none header accepted; signature is not verified.

// vuln-code-snippet start testcodeCrypto027
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let claims = decode_jwt_no_verify(&token); // vuln-code-snippet target-line testcodeCrypto027
    super::shared::BenchmarkResponse::ok(&format!("user={}", claims))
}

fn decode_jwt_no_verify(_token: &str) -> String {
    "user123".to_string()
}
// vuln-code-snippet end testcodeCrypto027
