//! CWE-601: User-supplied URL used directly as redirect destination without validation.

// vuln-code-snippet start testcodeRedirect021
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let location = format!("Location: {}", url); // vuln-code-snippet target-line testcodeRedirect021
    super::shared::BenchmarkResponse::ok(&location)
}
// vuln-code-snippet end testcodeRedirect021
