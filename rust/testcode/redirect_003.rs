//! CWE-601: return_to query parameter used directly as 302 redirect destination.

// vuln-code-snippet start testcodeRedirect003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let return_to = req.param("return_to");
    let header = format!("Location: {}", return_to); // vuln-code-snippet target-line testcodeRedirect003
    super::shared::BenchmarkResponse::ok(&format!("302 {}", header))
}
// vuln-code-snippet end testcodeRedirect003
