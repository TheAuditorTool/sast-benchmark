//! CWE-614: Session cookie uses SameSite=Lax without Secure or HttpOnly flags.

// vuln-code-snippet start testcodeSecurecookie032
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let cookie = format!("session={}; SameSite=Lax", token); // vuln-code-snippet target-line testcodeSecurecookie032
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie032
