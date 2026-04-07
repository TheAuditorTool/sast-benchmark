//! CWE-614: Constant expression always selects secure cookie branch; unsafe branch never reached.

// vuln-code-snippet start testcodeSecurecookie049
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    if 1 + 1 == 2 {
        let cookie = format!("session={}; Secure; HttpOnly; SameSite=Strict", token); // vuln-code-snippet target-line testcodeSecurecookie049
        super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
    } else {
        let cookie = format!("session={}", token);
        super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
    }
}
// vuln-code-snippet end testcodeSecurecookie049
