//! CWE-601: Slash prefix check bypassed by protocol-relative URL starting with //.

// vuln-code-snippet start testcodeRedirect032
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    if url.starts_with('/') {
        let location = format!("Location: {}", url); // vuln-code-snippet target-line testcodeRedirect032
        super::shared::BenchmarkResponse::ok(&location)
    } else {
        super::shared::BenchmarkResponse::bad_request("Must be relative")
    }
}
// vuln-code-snippet end testcodeRedirect032
