//! CWE-614: Constant-folded condition always produces secure cookie; insecure path is unreachable.

// vuln-code-snippet start testcodeSecurecookie046
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let cookie = if 6 * 7 == 42 {
        format!("session={}; Secure; HttpOnly; SameSite=Strict", token) // vuln-code-snippet target-line testcodeSecurecookie046
    } else {
        format!("session={}", token)
    };
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie046
