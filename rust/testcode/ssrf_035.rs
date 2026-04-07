//! CWE-918: Fetch function ignores user-supplied URL and always fetches a hardcoded endpoint.

// vuln-code-snippet start testcodeSsrf035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_url = req.param("url");

    let _resp = fetch_status(&user_url); // vuln-code-snippet target-line testcodeSsrf035

    super::shared::BenchmarkResponse::ok("Status retrieved")
}

fn fetch_status(_user_url: &str) -> String {
    simulated_reqwest_get("https://api.example.com/status")
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf035
