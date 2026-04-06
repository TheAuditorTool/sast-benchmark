//! CWE-614: Cookie added to jar using plain constructor without security flags.

// vuln-code-snippet start testcodeSecurecookie003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let value = req.param("session_id");

    // Simulates: rocket::http::Cookie::new("auth", &value) via CookieJar::add()
    let cookie = cookie_new("auth", &value); // vuln-code-snippet target-line testcodeSecurecookie003

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}

fn cookie_new(name: &str, value: &str) -> String {
    // Simulates: Cookie::new() -- no flags set
    format!("{}={}", name, value)
}
// vuln-code-snippet end testcodeSecurecookie003
