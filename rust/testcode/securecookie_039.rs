//! CWE-614: Long-lived session cookie includes all required security attributes.

// vuln-code-snippet start testcodeSecurecookie039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let cookie = format!("session={}; Path=/; Secure; HttpOnly; SameSite=Strict; Max-Age=86400", token); // vuln-code-snippet target-line testcodeSecurecookie039
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie039
