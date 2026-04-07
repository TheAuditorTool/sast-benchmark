//! CWE-918: Length check only on URL — no domain validation, SSRF still possible.

// vuln-code-snippet start testcodeSsrf022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    if url.len() >= 200 {
        return super::shared::BenchmarkResponse::bad_request("URL too long");
    }

    let _resp = simulated_reqwest_get(&url); // vuln-code-snippet target-line testcodeSsrf022

    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf022
