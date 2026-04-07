struct WebhookReq {
    target: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let wh = WebhookReq {
        target: req.param("url"),
    };

    let _resp = simulated_http_post(&wh.target, "payload");

    super::shared::BenchmarkResponse::ok("Webhook dispatched")
}

fn simulated_http_post(url: &str, body: &str) -> String {
    format!("POST {} body={}", url, body)
}
