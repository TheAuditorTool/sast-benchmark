//! SSRF True Positive — CWE-918
//! DNS rebinding: URL resolves externally first, then to internal on second
//! resolution. No post-resolution IP check performed.

// vuln-code-snippet start testcodeSsrf002Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    // VULNERABLE: Fetch without post-resolve IP check — DNS rebinding possible
    let _resp = fetch_without_resolve_check(&url); // vuln-code-snippet vuln-line testcodeSsrf002Vulnerable

    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn fetch_without_resolve_check(url: &str) -> String {
    // In production: reqwest::get(url).await — no IP validation after DNS resolves
    format!("Response from {} (no post-resolve check)", url)
}
// vuln-code-snippet end testcodeSsrf002Vulnerable
