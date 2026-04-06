//! CWE-614: Session cookie constructed without Secure or HttpOnly attributes.

// vuln-code-snippet start testcodeSecurecookie001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    // Simulates: actix_web::cookie::Cookie::build("session", &token).finish()
    let cookie = format!("session={}", token); // vuln-code-snippet target-line testcodeSecurecookie001

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie001
