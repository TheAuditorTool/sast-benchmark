//! CWE-79: User input injected into JavaScript context without escaping.

// vuln-code-snippet start testcodeXss004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_name = req.param("name");

    let html = format!("<html><script>var name='{}';</script><body>Hi</body></html>", user_name); // vuln-code-snippet target-line testcodeXss004

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss004
