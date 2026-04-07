//! CWE-918: User-supplied URL fetched directly without any validation.

// vuln-code-snippet start testcodeSsrf014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let _resp = simulated_reqwest_get(&url); // vuln-code-snippet target-line testcodeSsrf014

    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf014
