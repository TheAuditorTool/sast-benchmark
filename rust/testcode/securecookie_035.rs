//! CWE-614: Privileged admin session cookie set without any security attributes.

// vuln-code-snippet start testcodeSecurecookie035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("admin_token");
    let cookie = format!("admin_session={}", token); // vuln-code-snippet target-line testcodeSecurecookie035
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie035
