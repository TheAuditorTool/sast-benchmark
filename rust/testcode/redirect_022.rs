//! CWE-601: Post-login redirect destination taken from user-supplied next parameter.

// vuln-code-snippet start testcodeRedirect022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let next = req.param("next");
    let location = format!("Location: {}", next); // vuln-code-snippet target-line testcodeRedirect022
    super::shared::BenchmarkResponse::ok(&location)
}
// vuln-code-snippet end testcodeRedirect022
