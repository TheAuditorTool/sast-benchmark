//! CWE-601: return_to query parameter forwarded as redirect destination without validation.

// vuln-code-snippet start testcodeRedirect024
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let return_to = req.param("return_to");
    let location = format!("Location: {}", return_to); // vuln-code-snippet target-line testcodeRedirect024
    super::shared::BenchmarkResponse::ok(&location)
}
// vuln-code-snippet end testcodeRedirect024
