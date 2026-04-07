pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let target = req.param("target");
    let proxy_url = format!("https://api.example.com/proxy?url={}", target);

    let _resp = simulated_reqwest_get(&proxy_url);

    super::shared::BenchmarkResponse::ok(&format!("Proxied: {}", target))
}

fn simulated_reqwest_get(url: &str) -> String {
    format!("Response from {}", url)
}
