//! CWE-614: Cookie built with Secure, HttpOnly, and SameSite=Strict attributes.

// vuln-code-snippet start testcodeSecurecookie011
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    // Simulates: Cookie::build("session", &token).secure(true).http_only(true).same_site(SameSite::Strict).finish()
    let cookie = cookie_build_all_flags("session", &token); // vuln-code-snippet target-line testcodeSecurecookie011

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn cookie_build_all_flags(name: &str, value: &str) -> String {
    format!("{}={}; Path=/; Secure; HttpOnly; SameSite=Strict", name, value)
}
// vuln-code-snippet end testcodeSecurecookie011
