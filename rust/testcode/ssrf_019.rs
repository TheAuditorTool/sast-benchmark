//! CWE-918: Scheme-only check allows SSRF to internal HTTPS endpoints.

// vuln-code-snippet start testcodeSsrf019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    if !url.starts_with("https://") {
        return super::shared::BenchmarkResponse::forbidden("Only HTTPS allowed");
    }

    let _resp = simulated_reqwest_get(&url); // vuln-code-snippet target-line testcodeSsrf019

    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf019
