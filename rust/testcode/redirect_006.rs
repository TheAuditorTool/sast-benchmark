//! CWE-601: URL starts_with("/") check that //evil.com bypasses.

// vuln-code-snippet start testcodeRedirect006
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    if url.starts_with("/") { // vuln-code-snippet target-line testcodeRedirect006
        super::shared::BenchmarkResponse::ok(&format!("Location: {}", url))
    } else {
        super::shared::BenchmarkResponse::bad_request("Must be relative URL")
    }
}
// vuln-code-snippet end testcodeRedirect006
