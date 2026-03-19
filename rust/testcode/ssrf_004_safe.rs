//! SSRF True Negative — CWE-918
//! No redirect following. Redirect policy set to none, preventing
//! attacker from bouncing to internal services.

// vuln-code-snippet start testcodeSsrf004Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    // SAFE: Redirect policy set to none — no redirect following
    let _resp = fetch_no_redirects(&url); // vuln-code-snippet safe-line testcodeSsrf004Safe

    super::shared::BenchmarkResponse::ok(&format!("Fetched (no redirects): {}", url))
}

fn fetch_no_redirects(url: &str) -> String {
    // In production: reqwest::Client::builder().redirect(Policy::none()).build().get(url).send()
    format!("Response from {} (redirects disabled)", url)
}
// vuln-code-snippet end testcodeSsrf004Safe
