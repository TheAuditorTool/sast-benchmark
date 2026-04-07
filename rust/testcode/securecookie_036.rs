//! CWE-614: Session cookie includes Secure, HttpOnly, and SameSite=Strict attributes.

// vuln-code-snippet start testcodeSecurecookie036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let cookie = format!("session={}; Path=/; Secure; HttpOnly; SameSite=Strict", token); // vuln-code-snippet target-line testcodeSecurecookie036
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie036
