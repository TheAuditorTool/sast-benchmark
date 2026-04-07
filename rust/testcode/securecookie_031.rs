//! CWE-614: Auth cookie constructed without HttpOnly or Secure flags despite short lifetime.

// vuln-code-snippet start testcodeSecurecookie031
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let cookie = format!("auth={}; Max-Age=1", token); // vuln-code-snippet target-line testcodeSecurecookie031
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie031
