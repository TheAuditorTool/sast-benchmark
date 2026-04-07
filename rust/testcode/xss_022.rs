//! CWE-79: User input injected into an HTML event handler attribute.

// vuln-code-snippet start testcodeXss022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val = req.param("val");

    let html = format!("<input type='text' onchange='validate(\"{}\")'>", val); // vuln-code-snippet target-line testcodeXss022

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss022
