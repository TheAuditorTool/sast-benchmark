//! CWE-614: Cookie headers assembled into Vec; session cookie in list lacks Secure and HttpOnly.

// vuln-code-snippet start testcodeSecurecookie029
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let mut cookies = Vec::new();
    cookies.push(format!("session={}; Path=/", token));
    cookies.push("lang=en; Path=/; Max-Age=86400".to_string());
    let session_cookie = &cookies[0]; // vuln-code-snippet target-line testcodeSecurecookie029
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", session_cookie))
}
// vuln-code-snippet end testcodeSecurecookie029
