//! CWE-614: Cookie value constructed via format! then all security attributes appended.

// vuln-code-snippet start testcodeSecurecookie043
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let token = req.param("token");
    let cookie_val = format!("{}-{}", user_id, token);
    let header = format!("session={}; Path=/; Secure; HttpOnly; SameSite=Strict", cookie_val); // vuln-code-snippet target-line testcodeSecurecookie043
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", header))
}
// vuln-code-snippet end testcodeSecurecookie043
