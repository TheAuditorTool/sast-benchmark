pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw_url = req.param("callback");
    let allowed_hosts = ["hooks.example.com", "webhooks.partner.com"];
    let host = raw_url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .split('/')
        .next()
        .unwrap_or("");
    if !allowed_hosts.contains(&host) {
        return super::shared::BenchmarkResponse::forbidden("Host not allowed");
    }
    super::shared::BenchmarkResponse::ok(&format!("Callback registered: {}", raw_url))
}
