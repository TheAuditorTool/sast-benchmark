//! CWE-918: HashMap stores both tainted and safe URLs — only safe key is read for fetch.

use std::collections::HashMap;

// vuln-code-snippet start testcodeSsrf032
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut m: HashMap<&str, String> = HashMap::new();
    m.insert("user_url", req.param("url"));
    m.insert("safe_url", "https://api.example.com/data".to_string());

    let url = m.get("safe_url").unwrap();

    let _resp = simulated_reqwest_get(url); // vuln-code-snippet target-line testcodeSsrf032

    super::shared::BenchmarkResponse::ok("Fetched")
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf032
