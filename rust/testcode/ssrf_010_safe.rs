//! SSRF True Negative — CWE-918
//! Proxy-only access. All outbound requests routed through an approved
//! proxy server that enforces its own access controls.

// vuln-code-snippet start testcodeSsrf010Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let proxy = "http://approved-proxy.internal:3128";
    // SAFE: All requests go through approved proxy
    let _resp = fetch_via_proxy(&url, proxy); // vuln-code-snippet safe-line testcodeSsrf010Safe

    super::shared::BenchmarkResponse::ok(&format!("Fetched via proxy: {}", url))
}

fn fetch_via_proxy(url: &str, proxy: &str) -> String {
    // In production: reqwest::Client::builder().proxy(Proxy::all(proxy)).build().get(url).send()
    format!("Response from {} via {}", url, proxy)
}
// vuln-code-snippet end testcodeSsrf010Safe
