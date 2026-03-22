//! CWE-79: User input placed in HTML attribute without escaping.

// vuln-code-snippet start testcodeXss003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_url = req.param("avatar_url");

    let html = format!("<html><body><img src='{}' alt='avatar'></body></html>", user_url); // vuln-code-snippet target-line testcodeXss003

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss003
