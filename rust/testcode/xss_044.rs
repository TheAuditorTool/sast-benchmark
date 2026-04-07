//! CWE-79: Plain-text response format; browser will not interpret content as HTML or execute scripts.

// vuln-code-snippet start testcodeXss044
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    // Simulates Content-Type: text/plain — browser will not execute scripts.
    let body = format!("Welcome: {}", name); // vuln-code-snippet target-line testcodeXss044

    super::shared::BenchmarkResponse::ok(&body)
}
// vuln-code-snippet end testcodeXss044
