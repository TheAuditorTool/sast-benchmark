//! CWE-614: Cookie set with SameSite=None but missing required Secure attribute.

// vuln-code-snippet start testcodeSecurecookie004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    let cookie = format!("session={}; Path=/; SameSite=None", token); // vuln-code-snippet target-line testcodeSecurecookie004

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie004
