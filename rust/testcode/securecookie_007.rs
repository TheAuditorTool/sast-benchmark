//! CWE-614: Remember-me cookie with predictable value and no security flags.

// vuln-code-snippet start testcodeSecurecookie007
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");

    let remember_value = format!("remember_{}", user_id);
    let cookie = format!("remember_me={}", remember_value); // vuln-code-snippet target-line testcodeSecurecookie007

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie007
