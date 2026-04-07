//! CWE-601: dest parameter forwarded as redirect destination with no validation.

// vuln-code-snippet start testcodeRedirect035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dest = req.param("dest");
    let location = format!("Location: {}", dest); // vuln-code-snippet target-line testcodeRedirect035
    super::shared::BenchmarkResponse::ok(&location)
}
// vuln-code-snippet end testcodeRedirect035
