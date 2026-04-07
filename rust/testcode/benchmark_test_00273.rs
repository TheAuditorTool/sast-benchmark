pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let callback = req.param("callback");

    let _resp = simulated_http_post(&callback, "{\"event\": \"done\"}");

    super::shared::BenchmarkResponse::ok("Webhook delivered")
}

fn simulated_http_post(url: &str, body: &str) -> String {
    format!("POST {} body={}", url, body)
}
