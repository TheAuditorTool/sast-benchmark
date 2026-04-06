//! CWE-614: JWT stored in cookie with Secure, HttpOnly, and SameSite=Strict.

// vuln-code-snippet start testcodeSecurecookie018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");

    let jwt = format!("header.{}.signature", user_id);
    let cookie = format!("jwt={}; Path=/; Secure; HttpOnly; SameSite=Strict", jwt); // vuln-code-snippet target-line testcodeSecurecookie018

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie018
