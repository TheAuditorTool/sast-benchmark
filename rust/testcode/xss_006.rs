//! CWE-79: User input JSON-encoded before embedding in script context.

// vuln-code-snippet start testcodeXss006
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_name = req.param("name");

    let json_safe = user_name
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('\'', "\\u0027"); // vuln-code-snippet target-line testcodeXss006

    let html = format!("<html><script>var name=\"{}\";</script><body>Hi</body></html>", json_safe);
    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss006
