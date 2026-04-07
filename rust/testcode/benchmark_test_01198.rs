pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let webhook = req.param("webhook_url");
    let payload = req.body_str();
    super::shared::BenchmarkResponse::ok(&format!("Sending payload to {}: {}", webhook, payload))
}
