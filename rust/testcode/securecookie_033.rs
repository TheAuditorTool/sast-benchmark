//! CWE-614: Long-lived remember-me cookie constructed without Secure or HttpOnly attributes.

// vuln-code-snippet start testcodeSecurecookie033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("remember_token");
    let cookie = format!("remember={}", token); // vuln-code-snippet target-line testcodeSecurecookie033
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie033
