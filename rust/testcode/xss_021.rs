//! CWE-79: User input interpolated directly into a JavaScript string literal in a script block.

// vuln-code-snippet start testcodeXss021
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");

    let html = format!("<script>var user = \"{}\";</script>", username); // vuln-code-snippet target-line testcodeXss021

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss021
