//! CWE-614: Authentication cookie constructed without HttpOnly flag, accessible to JavaScript.

// vuln-code-snippet start testcodeSecurecookie022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let cookie = format!("auth={}; Path=/; Secure", token); // vuln-code-snippet target-line testcodeSecurecookie022
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie022
