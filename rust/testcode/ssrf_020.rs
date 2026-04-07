//! CWE-918: User-controlled endpoint parameter fetched directly.

// vuln-code-snippet start testcodeSsrf020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");

    let _resp = simulated_reqwest_get(&endpoint); // vuln-code-snippet target-line testcodeSsrf020

    super::shared::BenchmarkResponse::ok(&format!("Called endpoint: {}", endpoint))
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf020
