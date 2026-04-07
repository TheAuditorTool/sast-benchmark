//! CWE-918: User-controlled service host embedded into URL with hardcoded path — host not validated.

// vuln-code-snippet start testcodeSsrf024
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let service = req.param("service");
    let url = format!("http://{}/health", service);

    let _resp = simulated_reqwest_get(&url); // vuln-code-snippet target-line testcodeSsrf024

    super::shared::BenchmarkResponse::ok(&format!("Health checked: {}", service))
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf024
