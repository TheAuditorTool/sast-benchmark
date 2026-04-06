//! CWE-614: CSRF token cookie configured with SameSite=Strict and Secure.

// vuln-code-snippet start testcodeSecurecookie019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    let csrf_token = "a1b2c3d4e5f6";
    let cookie = format!("csrf={}; Path=/; Secure; SameSite=Strict", csrf_token); // vuln-code-snippet target-line testcodeSecurecookie019

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie019
