//! CWE-614: CSRF token cookie includes Secure and SameSite=Strict for proper protection.

// vuln-code-snippet start testcodeSecurecookie045
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let csrf = req.param("csrf");
    let cookie = format!("csrf_token={}; Path=/; Secure; SameSite=Strict", csrf); // vuln-code-snippet target-line testcodeSecurecookie045
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie045
