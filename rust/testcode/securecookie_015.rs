//! CWE-614: Short-lived session cookie with all security flags and 15-minute Max-Age.

// vuln-code-snippet start testcodeSecurecookie015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    let cookie = format!("session={}; Path=/; Max-Age=900; Secure; HttpOnly; SameSite=Strict", token); // vuln-code-snippet target-line testcodeSecurecookie015

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie015
