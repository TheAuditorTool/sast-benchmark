//! CWE-614: Cookie builder ignores user-supplied token and returns a static secure cookie.

// vuln-code-snippet start testcodeSecurecookie048
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _token = req.param("token");
    let cookie = build_static_cookie(&_token); // vuln-code-snippet target-line testcodeSecurecookie048
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn build_static_cookie(_user_input: &str) -> &'static str {
    "session=static-token; Secure; HttpOnly; SameSite=Strict"
}
// vuln-code-snippet end testcodeSecurecookie048
