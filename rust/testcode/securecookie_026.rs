//! CWE-614: Cookie configuration struct lacks security flags; resulting cookie is insecure.

// vuln-code-snippet start testcodeSecurecookie026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let config = CookieConfig {
        name: "session".to_string(),
        value: req.param("token"),
        secure: false,
        http_only: false,
    };
    let cookie = build_cookie(&config); // vuln-code-snippet target-line testcodeSecurecookie026
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

struct CookieConfig { name: String, value: String, secure: bool, http_only: bool }

fn build_cookie(c: &CookieConfig) -> String {
    let mut s = format!("{}={}", c.name, c.value);
    if c.secure { s.push_str("; Secure"); }
    if c.http_only { s.push_str("; HttpOnly"); }
    s
}
// vuln-code-snippet end testcodeSecurecookie026
