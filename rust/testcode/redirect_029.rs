//! CWE-601: OAuth callback_url parameter used as redirect without host validation.

// vuln-code-snippet start testcodeRedirect029
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let callback = req.param("callback_url");
    let code = req.param("code");
    let location = format!("Location: {}?code={}", callback, code); // vuln-code-snippet target-line testcodeRedirect029
    super::shared::BenchmarkResponse::ok(&location)
}
// vuln-code-snippet end testcodeRedirect029
