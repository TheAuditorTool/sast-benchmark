//! Cross-Site Scripting True Positive — CWE-79
//! User input injected into JavaScript context without escaping. Attacker
//! can break out with `'; alert(document.cookie); //` to execute arbitrary JS.

// vuln-code-snippet start testcodeXss004Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_name = req.param("name");

    // VULNERABLE: Unescaped user input in script block — JS injection
    let html = format!("<html><script>var name='{}';</script><body>Hi</body></html>", user_name); // vuln-code-snippet vuln-line testcodeXss004Vulnerable

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss004Vulnerable
