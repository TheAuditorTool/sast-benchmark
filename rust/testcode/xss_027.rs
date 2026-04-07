//! CWE-79: User input injected into a CSS style attribute value allowing CSS injection.

// vuln-code-snippet start testcodeXss027
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let color = req.param("color");

    let html = format!("<div style=\"color: {}\">text</div>", color); // vuln-code-snippet target-line testcodeXss027

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss027
