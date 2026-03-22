//! CWE-918: User URL fetched with redirect following enabled.

// vuln-code-snippet start testcodeSsrf001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let _resp = simulated_client_get_with_redirects(&url); // vuln-code-snippet target-line testcodeSsrf001

    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn simulated_client_get_with_redirects(url: &str) -> String {
    // In production: reqwest::Client::builder().redirect(Policy::default()).build().get(url).send()
    format!("Response from {} (redirects followed)", url)
}
// vuln-code-snippet end testcodeSsrf001
