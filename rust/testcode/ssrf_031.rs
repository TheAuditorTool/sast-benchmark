//! CWE-918: Tainted variable overwritten with hardcoded safe URL before fetch.

// vuln-code-snippet start testcodeSsrf031
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut url = req.param("url");
    url = "https://api.example.com/status".to_string();

    let _resp = simulated_reqwest_get(&url); // vuln-code-snippet target-line testcodeSsrf031

    super::shared::BenchmarkResponse::ok("Status fetched")
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf031
