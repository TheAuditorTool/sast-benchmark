//! CWE-614: Cookie with 365-day Max-Age set without Secure flag.

// vuln-code-snippet start testcodeSecurecookie005
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    let max_age = 365 * 24 * 60 * 60;
    let cookie = format!("session={}; Path=/; Max-Age={}", token, max_age); // vuln-code-snippet target-line testcodeSecurecookie005

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie005
