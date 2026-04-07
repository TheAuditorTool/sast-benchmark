//! CWE-601: Domain prefix check can be bypassed with subdomain confusion like example.com.evil.com.

// vuln-code-snippet start testcodeRedirect031
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    if url.contains("example.com") {
        let location = format!("Location: {}", url); // vuln-code-snippet target-line testcodeRedirect031
        super::shared::BenchmarkResponse::ok(&location)
    } else {
        super::shared::BenchmarkResponse::bad_request("Invalid redirect")
    }
}
// vuln-code-snippet end testcodeRedirect031
