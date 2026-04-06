//! CWE-614: Cookie value encrypted before storage using cookie jar encryption API.

// vuln-code-snippet start testcodeSecurecookie017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let value = req.param("data");

    // Simulates: cookie::CookieJar::encrypted(&key).add(cookie)
    let encrypted_cookie = encrypted_jar_add("session", &value); // vuln-code-snippet target-line testcodeSecurecookie017

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", encrypted_cookie))
}

fn encrypted_jar_add(name: &str, value: &str) -> String {
    // Simulates: cookie::CookieJar::encrypted() -- encrypts value, sets Secure
    let encrypted = format!("enc_{}", value);
    format!("{}={}; Path=/; Secure; HttpOnly", name, encrypted)
}
// vuln-code-snippet end testcodeSecurecookie017
