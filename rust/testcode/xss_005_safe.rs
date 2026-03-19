//! Cross-Site Scripting True Negative — CWE-79
//! User input URL-encoded before placement in HTML attribute.
//! Percent-encoding prevents attribute breakout and script injection.

// vuln-code-snippet start testcodeXss005Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_url = req.param("avatar_url");

    // SAFE: URL-encoding prevents attribute injection
    let encoded: String = user_url.bytes().map(|b| match b {
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
            String::from(b as char)
        }
        _ => format!("%{:02X}", b),
    }).collect(); // vuln-code-snippet safe-line testcodeXss005Safe

    let html = format!("<html><body><img src='{}' alt='avatar'></body></html>", encoded);
    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss005Safe
