//! CWE-614: CSRF token cookie set without Secure flag, exposable over non-TLS connections.

// vuln-code-snippet start testcodeSecurecookie034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let csrf = req.param("csrf");
    let cookie = format!("csrf_token={}; SameSite=Strict", csrf); // vuln-code-snippet target-line testcodeSecurecookie034
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie034
