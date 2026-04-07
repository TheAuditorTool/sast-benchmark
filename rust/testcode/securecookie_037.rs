//! CWE-614: Auth cookie includes Secure, HttpOnly, and SameSite=Strict preventing common attacks.

// vuln-code-snippet start testcodeSecurecookie037
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let cookie = format!("auth={}; Path=/; Secure; HttpOnly; SameSite=Strict; Max-Age=3600", token); // vuln-code-snippet target-line testcodeSecurecookie037
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie037
