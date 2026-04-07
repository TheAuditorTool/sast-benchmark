//! CWE-918: Referer header used as outbound fetch target without validation.

// vuln-code-snippet start testcodeSsrf026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let referer = req.header("Referer");

    let _resp = simulated_reqwest_get(&referer); // vuln-code-snippet target-line testcodeSsrf026

    super::shared::BenchmarkResponse::ok(&format!("Fetched referer: {}", referer))
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf026
