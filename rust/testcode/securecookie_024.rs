//! CWE-614: Cookie uses SameSite=None without Secure flag, violating the SameSite=None requirement.

// vuln-code-snippet start testcodeSecurecookie024
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let cookie = format!("session={}; Path=/; SameSite=None; HttpOnly", token); // vuln-code-snippet target-line testcodeSecurecookie024
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie024
