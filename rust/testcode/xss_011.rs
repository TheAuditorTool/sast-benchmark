//! CWE-79: Response served as text/plain.

// vuln-code-snippet start testcodeXss011
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let body = format!("Hello, {}! This is a plain text response.", name); // vuln-code-snippet target-line testcodeXss011

    let mut resp = super::shared::BenchmarkResponse::ok(&body);
    resp.body = format!("Content-Type: text/plain\n\n{}", body);
    resp
}
// vuln-code-snippet end testcodeXss011
