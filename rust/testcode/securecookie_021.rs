//! CWE-614: Session cookie constructed without Secure or HttpOnly attributes.

// vuln-code-snippet start testcodeSecurecookie021
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let cookie = format!("session={}; Path=/", token); // vuln-code-snippet target-line testcodeSecurecookie021
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie021
