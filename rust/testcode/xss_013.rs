//! CWE-79: User input placed in onclick event handler attribute.

// vuln-code-snippet start testcodeXss013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");

    let html = format!(
        "<html><body><button onclick='doAction(\"{}\")'>Go</button></body></html>",
        action
    ); // vuln-code-snippet target-line testcodeXss013

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss013
