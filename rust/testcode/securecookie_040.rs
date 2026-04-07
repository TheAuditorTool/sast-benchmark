//! CWE-614: Admin session cookie includes Secure, HttpOnly, and SameSite=Strict.

// vuln-code-snippet start testcodeSecurecookie040
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("admin_token");
    let cookie = format!("admin={}; Path=/admin; Secure; HttpOnly; SameSite=Strict", token); // vuln-code-snippet target-line testcodeSecurecookie040
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie040
