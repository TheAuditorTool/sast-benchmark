//! CWE-614: Session cookie with Path=/ but no domain restriction or Secure flag.

// vuln-code-snippet start testcodeSecurecookie009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    let cookie = build_session_cookie(&token); // vuln-code-snippet target-line testcodeSecurecookie009

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn build_session_cookie(value: &str) -> String {
    format!("session={}; Path=/; HttpOnly", value)
}
// vuln-code-snippet end testcodeSecurecookie009
