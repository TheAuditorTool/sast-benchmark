//! CWE-918: Dead-code branch — constant condition always selects hardcoded URL.

// vuln-code-snippet start testcodeSsrf030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user_url = req.param("url");

    let target = if 7 * 6 > 40 {
        "https://api.example.com/data".to_string()
    } else {
        req.param("url")
    };

    let _resp = simulated_reqwest_get(&target); // vuln-code-snippet target-line testcodeSsrf030

    super::shared::BenchmarkResponse::ok("Fetched")
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf030
