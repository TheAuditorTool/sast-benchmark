//! CWE-614: Middleware that enforces security flags on every outgoing Set-Cookie header.

// vuln-code-snippet start testcodeSecurecookie020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("token");

    let raw_cookie = format!("session={}", token);
    let enforced = enforce_cookie_flags(&raw_cookie); // vuln-code-snippet target-line testcodeSecurecookie020

    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", enforced))
}

fn enforce_cookie_flags(cookie: &str) -> String {
    let mut result = cookie.to_string();
    if !result.contains("Secure") { result.push_str("; Secure"); }
    if !result.contains("HttpOnly") { result.push_str("; HttpOnly"); }
    if !result.contains("SameSite") { result.push_str("; SameSite=Lax"); }
    result
}
// vuln-code-snippet end testcodeSecurecookie020
