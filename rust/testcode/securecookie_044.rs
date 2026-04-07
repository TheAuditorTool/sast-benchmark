//! CWE-614: Session cookie in cookie list includes all required security attributes.

// vuln-code-snippet start testcodeSecurecookie044
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let mut cookies = Vec::new();
    cookies.push(format!("session={}; Path=/; Secure; HttpOnly; SameSite=Strict", token));
    cookies.push("lang=en; Path=/; Max-Age=86400".to_string());
    let session_cookie = &cookies[0]; // vuln-code-snippet target-line testcodeSecurecookie044
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", session_cookie))
}
// vuln-code-snippet end testcodeSecurecookie044
