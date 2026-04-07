//! CWE-918: User input embedded into proxy URL via format!, then fetched.

// vuln-code-snippet start testcodeSsrf016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let target = req.param("target");
    let proxy_url = format!("https://api.example.com/proxy?url={}", target);

    let _resp = simulated_reqwest_get(&proxy_url); // vuln-code-snippet target-line testcodeSsrf016

    super::shared::BenchmarkResponse::ok(&format!("Proxied: {}", target))
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf016
