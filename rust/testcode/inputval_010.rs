//! CWE-20: Username field accepted without maximum length restriction.

// vuln-code-snippet start testcodeInputval010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username"); // vuln-code-snippet target-line testcodeInputval010
    super::shared::BenchmarkResponse::ok(&format!("Registered: {}", username))
}
// vuln-code-snippet end testcodeInputval010
