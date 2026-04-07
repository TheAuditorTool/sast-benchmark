//! CWE-918: User-supplied RSS feed URL fetched without validation.

// vuln-code-snippet start testcodeSsrf023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let feed = req.param("feed");

    let _resp = simulated_reqwest_get(&feed); // vuln-code-snippet target-line testcodeSsrf023

    super::shared::BenchmarkResponse::ok(&format!("Fetched feed: {}", feed))
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf023
