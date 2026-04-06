//! CWE-614: Cookie uses __Host- prefix, enforcing Secure and Path=/ by browser policy.

// vuln-code-snippet start testcodeSecurecookie014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    let cookie = format!("__Host-session={}; Path=/; Secure; HttpOnly", token); // vuln-code-snippet target-line testcodeSecurecookie014

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie014
