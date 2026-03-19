//! Cross-Site Scripting True Negative — CWE-79
//! User input serialized via JSON encoding before embedding in script context.
//! JSON.stringify escapes quotes and special characters, preventing JS injection.

// vuln-code-snippet start testcodeXss006Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_name = req.param("name");

    // SAFE: JSON-encoding escapes quotes, backslashes, and control chars
    let json_safe = user_name
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('\'', "\\u0027"); // vuln-code-snippet safe-line testcodeXss006Safe

    let html = format!("<html><script>var name=\"{}\";</script><body>Hi</body></html>", json_safe);
    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss006Safe
