//! CWE-601: X-Forwarded-Host header used to construct redirect URL enabling header injection.

// vuln-code-snippet start testcodeRedirect034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let host = req.header("X-Forwarded-Host");
    let path = req.param("path");
    let location = format!("Location: https://{}{}", host, path); // vuln-code-snippet target-line testcodeRedirect034
    super::shared::BenchmarkResponse::ok(&location)
}
// vuln-code-snippet end testcodeRedirect034
