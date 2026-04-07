//! CWE-918: Taint flows through struct field into HTTP POST call.

// vuln-code-snippet start testcodeSsrf015
struct WebhookReq {
    target: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let wh = WebhookReq {
        target: req.param("url"),
    };

    let _resp = simulated_http_post(&wh.target, "payload"); // vuln-code-snippet target-line testcodeSsrf015

    super::shared::BenchmarkResponse::ok("Webhook dispatched")
}

fn simulated_http_post(url: &str, body: &str) -> String {
    // In production: reqwest::Client::new().post(url).body(body).send().await
    format!("POST {} body={}", url, body)
}
// vuln-code-snippet end testcodeSsrf015
