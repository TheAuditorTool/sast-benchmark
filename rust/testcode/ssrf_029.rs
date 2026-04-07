//! CWE-918: Hardcoded base URL with user-supplied path suffix validated as alphanumeric only.

// vuln-code-snippet start testcodeSsrf029
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let resource = req.param("resource");

    if !resource.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return super::shared::BenchmarkResponse::bad_request("Invalid resource identifier");
    }

    let url = format!("https://api.example.com/data/{}", resource);

    let _resp = simulated_reqwest_get(&url); // vuln-code-snippet target-line testcodeSsrf029

    super::shared::BenchmarkResponse::ok(&format!("Fetched resource: {}", resource))
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf029
