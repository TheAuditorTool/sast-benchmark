//! SSRF True Negative — CWE-918
//! Request timeout. 5-second max prevents slow-loris and port scanning
//! against internal services.

// vuln-code-snippet start testcodeSsrf005Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    // SAFE: Strict timeout prevents abuse of internal service scanning
    let _resp = fetch_with_timeout(&url, 5); // vuln-code-snippet safe-line testcodeSsrf005Safe

    super::shared::BenchmarkResponse::ok(&format!("Fetched with timeout: {}", url))
}

fn fetch_with_timeout(url: &str, secs: u64) -> String {
    // In production: reqwest::Client::builder().timeout(Duration::from_secs(secs)).build().get(url)
    format!("Response from {} (timeout {}s)", url, secs)
}
// vuln-code-snippet end testcodeSsrf005Safe
