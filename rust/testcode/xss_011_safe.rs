//! Cross-Site Scripting True Negative — CWE-79
//! Response served as text/plain. Browser renders content as literal text,
//! never interpreting HTML tags or executing scripts.

// vuln-code-snippet start testcodeXss011Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    // SAFE: Content-Type: text/plain — browser renders as literal text, no HTML parsing
    let body = format!("Hello, {}! This is a plain text response.", name); // vuln-code-snippet safe-line testcodeXss011Safe

    let mut resp = super::shared::BenchmarkResponse::ok(&body);
    resp.body = format!("Content-Type: text/plain\n\n{}", body);
    resp
}
// vuln-code-snippet end testcodeXss011Safe
