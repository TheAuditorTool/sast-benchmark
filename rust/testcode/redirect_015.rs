//! CWE-601: Same-origin check comparing parsed URL host to request Host header.

// vuln-code-snippet start testcodeRedirect015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let target = req.param("url");
    let request_host = req.header("host");
    let target_host = extract_host(&target);
    if target_host == request_host || target_host.is_empty() { // vuln-code-snippet target-line testcodeRedirect015
        super::shared::BenchmarkResponse::ok(&format!("Location: {}", target))
    } else {
        super::shared::BenchmarkResponse::bad_request("Cross-origin redirect blocked")
    }
}
fn extract_host(url: &str) -> String {
    url.split("://").nth(1).and_then(|s| s.split('/').next()).unwrap_or("").to_string()
}
// vuln-code-snippet end testcodeRedirect015
