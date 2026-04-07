//! CWE-918: Domain suffix check only — URL containing "example.com" anywhere passes, allowing bypass.

// vuln-code-snippet start testcodeSsrf025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    if !url.contains("example.com") {
        return super::shared::BenchmarkResponse::forbidden("Domain not allowed");
    }

    let _resp = simulated_reqwest_get(&url); // vuln-code-snippet target-line testcodeSsrf025

    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf025
