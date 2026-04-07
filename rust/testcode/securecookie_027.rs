//! CWE-614: Session cookie value assembled via format! with no Secure or HttpOnly attributes.

// vuln-code-snippet start testcodeSecurecookie027
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let token = req.param("token");
    let cookie_val = format!("{}-{}", user_id, token);
    let header = format!("session={}; Path=/", cookie_val); // vuln-code-snippet target-line testcodeSecurecookie027
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", header))
}
// vuln-code-snippet end testcodeSecurecookie027
