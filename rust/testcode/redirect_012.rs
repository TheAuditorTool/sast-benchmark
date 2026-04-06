//! CWE-601: Redirect to hardcoded path literal.

// vuln-code-snippet start testcodeRedirect012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");
    let target = "/dashboard"; // vuln-code-snippet target-line testcodeRedirect012
    super::shared::BenchmarkResponse::ok(&format!("Location: {}", target))
}
// vuln-code-snippet end testcodeRedirect012
