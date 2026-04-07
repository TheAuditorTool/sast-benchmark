//! CWE-79: Partial sanitization strips angle brackets but leaves double-quote injection via attribute context.

// vuln-code-snippet start testcodeXss028
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("val");

    let filtered = input.replace('<', "").replace('>', "");
    let html = format!("<input value=\"{}\">", filtered); // vuln-code-snippet target-line testcodeXss028

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss028
