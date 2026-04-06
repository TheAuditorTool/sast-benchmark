//! CWE-601: URL with userinfo component bypasses naive host check.

// vuln-code-snippet start testcodeRedirect008
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let target = req.param("url");
    if target.contains("example.com") {
        super::shared::BenchmarkResponse::ok(&format!("Location: {}", target)) // vuln-code-snippet target-line testcodeRedirect008
    } else {
        super::shared::BenchmarkResponse::bad_request("Invalid redirect")
    }
}
// vuln-code-snippet end testcodeRedirect008
