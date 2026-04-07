//! CWE-79: Length check is the only guard; input still reaches HTML unescaped.

// vuln-code-snippet start testcodeXss030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("text");

    if input.len() > 50 {
        return super::shared::BenchmarkResponse::bad_request("input too long");
    }

    let html = format!("<p>{}</p>", input); // vuln-code-snippet target-line testcodeXss030

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss030
