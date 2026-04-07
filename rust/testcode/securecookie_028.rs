//! CWE-614: Helper function constructs cookie without Secure or HttpOnly attributes.

// vuln-code-snippet start testcodeSecurecookie028
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let cookie = make_session_cookie(&token); // vuln-code-snippet target-line testcodeSecurecookie028
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn make_session_cookie(token: &str) -> String {
    format!("session={}; Path=/; Max-Age=3600", token)
}
// vuln-code-snippet end testcodeSecurecookie028
