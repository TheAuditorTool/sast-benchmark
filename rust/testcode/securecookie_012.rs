//! CWE-614: Cookie middleware configured with all security attributes enabled.

// vuln-code-snippet start testcodeSecurecookie012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    // Simulates: tower_cookies::CookieManagerLayer with secure defaults
    let cookie = tower_cookies_secure("session", &token); // vuln-code-snippet target-line testcodeSecurecookie012

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn tower_cookies_secure(name: &str, value: &str) -> String {
    // Simulates: tower-cookies middleware with security attributes
    format!("{}={}; Path=/; Secure; HttpOnly; SameSite=Lax", name, value)
}
// vuln-code-snippet end testcodeSecurecookie012
