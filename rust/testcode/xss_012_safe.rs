//! Cross-Site Scripting True Negative — CWE-79
//! Double-encoded output — HTML entities are encoded before rendering.
//! Even if attacker injects `&lt;script&gt;`, the second encoding renders it inert.

// vuln-code-snippet start testcodeXss012Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    // SAFE: Double-encoding — first pass escapes, second pass encodes the escapes
    let first_pass = name
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    let double_encoded = first_pass
        .replace('&', "&amp;"); // vuln-code-snippet safe-line testcodeXss012Safe

    let html = format!("<html><body><p>{}</p></body></html>", double_encoded);
    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss012Safe
