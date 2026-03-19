//! Cross-Site Scripting True Positive — CWE-79
//! User input interpolated directly into HTML response without escaping.

// vuln-code-snippet start testcodeXss001Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    // VULNERABLE: User input directly in HTML — <script>alert(1)</script> works
    let html = format!("<html><body><h1>Hello, {}!</h1></body></html>", name); // vuln-code-snippet vuln-line testcodeXss001Vulnerable

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss001Vulnerable
