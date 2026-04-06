//! CWE-601: Redirect rejected if URL contains :// or starts with //.

// vuln-code-snippet start testcodeRedirect014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let target = req.param("url");
    if target.contains("://") || target.starts_with("//") { // vuln-code-snippet target-line testcodeRedirect014
        return super::shared::BenchmarkResponse::bad_request("Absolute URLs not allowed");
    }
    super::shared::BenchmarkResponse::ok(&format!("Location: {}", target))
}
// vuln-code-snippet end testcodeRedirect014
