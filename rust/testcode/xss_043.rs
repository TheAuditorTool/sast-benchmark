//! CWE-79: User input embedded in a JSON response body; browser will not render it as HTML.

// vuln-code-snippet start testcodeXss043
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    // JSON response; browser will not render as HTML.
    let body = format!("{{\"message\": \"hello\", \"user\": \"{}\"}}", name); // vuln-code-snippet target-line testcodeXss043

    super::shared::BenchmarkResponse::ok(&body)
}
// vuln-code-snippet end testcodeXss043
