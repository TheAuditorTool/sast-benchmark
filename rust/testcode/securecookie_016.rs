//! CWE-614: actix-session middleware configured with all cookie security defaults.

// vuln-code-snippet start testcodeSecurecookie016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    // Simulates: actix_session::SessionMiddleware::builder(store, key).cookie_secure(true).build()
    let config = actix_session_config(); // vuln-code-snippet target-line testcodeSecurecookie016

    super::shared::BenchmarkResponse::ok(&format!("Session configured: {}", config))
}

fn actix_session_config() -> String {
    // Simulates: CookieConfiguration with secure=true, http_only=true, same_site=Strict
    "secure=true,http_only=true,same_site=strict".to_string()
}
// vuln-code-snippet end testcodeSecurecookie016
