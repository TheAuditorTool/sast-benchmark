//! CWE-614: Session cookie constructed without Secure flag, transmittable over plain HTTP.

// vuln-code-snippet start testcodeSecurecookie023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let cookie = format!("session={}; Path=/; HttpOnly", token); // vuln-code-snippet target-line testcodeSecurecookie023
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie023
