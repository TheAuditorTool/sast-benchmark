//! Cross-Site Scripting True Positive — CWE-79
//! User input placed in HTML attribute without escaping. Attacker can
//! break out of attribute with `" onload="alert(1)` or inject javascript: URI.

// vuln-code-snippet start testcodeXss003Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_url = req.param("avatar_url");

    // VULNERABLE: Unescaped user input in HTML attribute — attribute injection
    let html = format!("<html><body><img src='{}' alt='avatar'></body></html>", user_url); // vuln-code-snippet vuln-line testcodeXss003Vulnerable

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss003Vulnerable
