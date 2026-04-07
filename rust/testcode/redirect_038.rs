//! CWE-601: Redirect destination is hardcoded; user input not used in Location header.

// vuln-code-snippet start testcodeRedirect038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _url = req.param("url");
    let location = "Location: /dashboard"; // vuln-code-snippet target-line testcodeRedirect038
    super::shared::BenchmarkResponse::ok(location)
}
// vuln-code-snippet end testcodeRedirect038
