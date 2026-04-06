//! CWE-614: Set-Cookie header built as raw string without Secure flag.

// vuln-code-snippet start testcodeSecurecookie002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    let cookie = format!("session={}; Path=/; HttpOnly", token); // vuln-code-snippet target-line testcodeSecurecookie002

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie002
