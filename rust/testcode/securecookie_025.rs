//! CWE-614: Session token set in cookie with no security attributes.

// vuln-code-snippet start testcodeSecurecookie025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let cookie = format!("session={}", token); // vuln-code-snippet target-line testcodeSecurecookie025
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie025
