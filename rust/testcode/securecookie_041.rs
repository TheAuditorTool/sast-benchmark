//! CWE-614: Cookie helper always appends Secure, HttpOnly, SameSite=Strict flags.

// vuln-code-snippet start testcodeSecurecookie041
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let cookie = make_secure_cookie("session", &token); // vuln-code-snippet target-line testcodeSecurecookie041
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn make_secure_cookie(name: &str, value: &str) -> String {
    format!("{}={}; Path=/; Secure; HttpOnly; SameSite=Strict", name, value)
}
// vuln-code-snippet end testcodeSecurecookie041
