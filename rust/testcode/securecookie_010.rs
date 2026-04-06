//! CWE-614: Cookie builder with explicit secure(false) downgrade.

// vuln-code-snippet start testcodeSecurecookie010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    // Simulates: Cookie::build("session", &token).secure(false).finish()
    let cookie = cookie_build_insecure("session", &token); // vuln-code-snippet target-line testcodeSecurecookie010

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn cookie_build_insecure(name: &str, value: &str) -> String {
    // Simulates: .secure(false) explicitly set
    format!("{}={}; Path=/; HttpOnly", name, value)
}
// vuln-code-snippet end testcodeSecurecookie010
