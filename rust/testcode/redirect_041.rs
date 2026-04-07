//! CWE-601: URL host extracted and checked against trusted domain list before redirect.

// vuln-code-snippet start testcodeRedirect041
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let host = extract_host(&url);
    if is_allowed_host(&host) {
        let location = format!("Location: {}", url); // vuln-code-snippet target-line testcodeRedirect041
        super::shared::BenchmarkResponse::ok(&location)
    } else {
        super::shared::BenchmarkResponse::bad_request("Disallowed host")
    }
}

fn extract_host(url: &str) -> String {
    url.split("://").nth(1)
        .and_then(|s| s.split('/').next())
        .unwrap_or("")
        .to_string()
}

fn is_allowed_host(host: &str) -> bool {
    matches!(host, "example.com" | "api.example.com" | "cdn.example.com")
}
// vuln-code-snippet end testcodeRedirect041
