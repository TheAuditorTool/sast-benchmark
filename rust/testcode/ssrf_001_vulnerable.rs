//! SSRF True Positive — CWE-918
//! URL from user with redirect following. Client follows redirects, allowing
//! attacker to redirect from external URL to internal service.

// vuln-code-snippet start testcodeSsrf001Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    // VULNERABLE: User-controlled URL fetched with redirect following enabled
    let _resp = simulated_client_get_with_redirects(&url); // vuln-code-snippet vuln-line testcodeSsrf001Vulnerable

    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn simulated_client_get_with_redirects(url: &str) -> String {
    // In production: reqwest::Client::builder().redirect(Policy::default()).build().get(url).send()
    format!("Response from {} (redirects followed)", url)
}
// vuln-code-snippet end testcodeSsrf001Vulnerable
