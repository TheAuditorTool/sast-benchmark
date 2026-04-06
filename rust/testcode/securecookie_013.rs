//! CWE-614: Encrypted cookie added via Rocket CookieJar::add_private with TLS.

// vuln-code-snippet start testcodeSecurecookie013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let value = req.param("session_id");

    // Simulates: jar.add_private(Cookie::new("session", &value)) with TLS in Rocket.toml
    let cookie = rocket_add_private("session", &value); // vuln-code-snippet target-line testcodeSecurecookie013

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn rocket_add_private(name: &str, value: &str) -> String {
    // Simulates: Rocket encrypted cookie jar -- encrypts value, sets Secure when TLS configured
    format!("{}=encrypted_{}; Path=/; Secure; HttpOnly", name, value)
}
// vuln-code-snippet end testcodeSecurecookie013
