//! Cross-Site Scripting True Negative — CWE-79
//! Content-Security-Policy header blocks inline script execution.
//! Even if XSS payload reaches HTML, browser refuses to execute it.

// vuln-code-snippet start testcodeXss008Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    // SAFE: CSP header prevents inline script execution
    let csp = "default-src 'self'; script-src 'self'; object-src 'none'"; // vuln-code-snippet safe-line testcodeXss008Safe
    let html = format!("<html><body><h1>Hello, {}!</h1></body></html>", name);

    let mut resp = super::shared::BenchmarkResponse::ok(&html);
    resp.body = format!("Content-Security-Policy: {}\n\n{}", csp, html);
    resp
}
// vuln-code-snippet end testcodeXss008Safe
