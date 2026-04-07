//! CWE-79: User input embedded inside an SVG text element without escaping.

// vuln-code-snippet start testcodeXss029
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let label = req.param("label");

    let html = format!("<svg><text x='10' y='20'>{}</text></svg>", label); // vuln-code-snippet target-line testcodeXss029

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss029
