//! CWE-918: Fetch without post-resolve IP check. DNS rebinding possible.

// vuln-code-snippet start testcodeSsrf002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let _resp = fetch_without_resolve_check(&url); // vuln-code-snippet target-line testcodeSsrf002

    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn fetch_without_resolve_check(url: &str) -> String {
    // In production: reqwest::get(url).await — no IP validation after DNS resolves
    format!("Response from {} (no post-resolve check)", url)
}
// vuln-code-snippet end testcodeSsrf002
