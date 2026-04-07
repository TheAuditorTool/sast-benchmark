//! CWE-601: Redirect URL validated with exact host comparison against trusted domain.

// vuln-code-snippet start testcodeRedirect039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    if is_trusted_host(&url) {
        let location = format!("Location: {}", url); // vuln-code-snippet target-line testcodeRedirect039
        super::shared::BenchmarkResponse::ok(&location)
    } else {
        super::shared::BenchmarkResponse::bad_request("Untrusted host")
    }
}

fn is_trusted_host(url: &str) -> bool {
    url.starts_with("https://example.com/") || url.starts_with("https://app.example.com/")
}
// vuln-code-snippet end testcodeRedirect039
