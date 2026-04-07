//! CWE-601: Redirect destination validated against explicit allowlist of permitted URLs.

// vuln-code-snippet start testcodeRedirect036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let allowed = ["/dashboard", "/profile", "/settings"];
    if allowed.contains(&url.as_str()) {
        let location = format!("Location: {}", url); // vuln-code-snippet target-line testcodeRedirect036
        super::shared::BenchmarkResponse::ok(&location)
    } else {
        super::shared::BenchmarkResponse::bad_request("Invalid redirect")
    }
}
// vuln-code-snippet end testcodeRedirect036
