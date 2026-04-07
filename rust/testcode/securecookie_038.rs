//! CWE-614: Cookie builder explicitly sets Secure and HttpOnly to true.

// vuln-code-snippet start testcodeSecurecookie038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");
    let config = CookieConfig { name: "session".to_string(), value: token, secure: true, http_only: true };
    let cookie = build_cookie(&config); // vuln-code-snippet target-line testcodeSecurecookie038
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

struct CookieConfig { name: String, value: String, secure: bool, http_only: bool }

fn build_cookie(c: &CookieConfig) -> String {
    let mut s = format!("{}={}", c.name, c.value);
    if c.secure { s.push_str("; Secure"); }
    if c.http_only { s.push_str("; HttpOnly"); }
    s.push_str("; SameSite=Strict");
    s
}
// vuln-code-snippet end testcodeSecurecookie038
