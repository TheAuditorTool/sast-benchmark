//! CWE-601: User-controlled path appended via format! and used as redirect target.

// vuln-code-snippet start testcodeRedirect027
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");
    let full_url = format!("https://example.com/{}", path);
    let location = format!("Location: {}", full_url); // vuln-code-snippet target-line testcodeRedirect027
    super::shared::BenchmarkResponse::ok(&location)
}
// vuln-code-snippet end testcodeRedirect027
