//! CWE-601: HTTP Referer header used as redirect target without validation.

// vuln-code-snippet start testcodeRedirect023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let referer = req.header("Referer");
    let location = format!("Location: {}", referer); // vuln-code-snippet target-line testcodeRedirect023
    super::shared::BenchmarkResponse::ok(&location)
}
// vuln-code-snippet end testcodeRedirect023
