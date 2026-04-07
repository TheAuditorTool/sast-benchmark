//! CWE-614: Cookie value unconditionally overwritten with server-generated safe token.

// vuln-code-snippet start testcodeSecurecookie047
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut token = req.param("token");
    token = generate_safe_token();
    let cookie = format!("session={}; Secure; HttpOnly; SameSite=Strict", token); // vuln-code-snippet target-line testcodeSecurecookie047
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn generate_safe_token() -> String {
    "server-generated-token-abc123".to_string()
}
// vuln-code-snippet end testcodeSecurecookie047
