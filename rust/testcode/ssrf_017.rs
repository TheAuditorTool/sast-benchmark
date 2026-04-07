//! CWE-918: User-supplied callback URL receives webhook notification without validation.

// vuln-code-snippet start testcodeSsrf017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let callback = req.param("callback");

    let _resp = simulated_http_post(&callback, "{\"event\": \"done\"}"); // vuln-code-snippet target-line testcodeSsrf017

    super::shared::BenchmarkResponse::ok("Webhook delivered")
}

fn simulated_http_post(url: &str, body: &str) -> String {
    // In production: reqwest::Client::new().post(url).body(body).send().await
    format!("POST {} body={}", url, body)
}
// vuln-code-snippet end testcodeSsrf017
