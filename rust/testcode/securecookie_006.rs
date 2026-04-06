//! CWE-614: Auth token cookie created without HttpOnly, accessible via JavaScript.

// vuln-code-snippet start testcodeSecurecookie006
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    // Simulates: Cookie::build("auth_token", &token).secure(true).finish() -- missing http_only
    let cookie = format!("auth_token={}; Path=/; Secure", token); // vuln-code-snippet target-line testcodeSecurecookie006

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie006
