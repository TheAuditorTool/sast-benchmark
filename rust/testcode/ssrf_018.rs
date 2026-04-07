//! CWE-918: Hostname extracted from user input, embedded into URL, fetched without validation.

// vuln-code-snippet start testcodeSsrf018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let host = req.param("host");
    let url = format!("http://{}/api/data", host);

    let _resp = simulated_reqwest_get(&url); // vuln-code-snippet target-line testcodeSsrf018

    super::shared::BenchmarkResponse::ok(&format!("Fetched from host: {}", host))
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf018
