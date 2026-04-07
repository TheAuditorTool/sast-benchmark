//! CWE-79: Tainted value flows through an intermediate format! before being embedded in HTML.

// vuln-code-snippet start testcodeXss026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let greeting = format!("Hello, {}!", name);
    let html = format!("<p>{}</p>", greeting); // vuln-code-snippet target-line testcodeXss026

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss026
