//! CWE-614: Insecure cookie stored under different HashMap key; secure cookie value is used.

use std::collections::HashMap;

// vuln-code-snippet start testcodeSecurecookie050
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut cookies = HashMap::new();
    let token = req.param("token");
    cookies.insert("raw", format!("session={}", token));
    cookies.insert("secure", format!("session={}; Secure; HttpOnly; SameSite=Strict", token));
    let cookie = cookies.get("secure").unwrap(); // vuln-code-snippet target-line testcodeSecurecookie050
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", cookie))
}
// vuln-code-snippet end testcodeSecurecookie050
