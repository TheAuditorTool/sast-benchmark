pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut url = req.param("url");
    url = "https://api.example.com/status".to_string();

    let _resp = simulated_reqwest_get(&url);

    super::shared::BenchmarkResponse::ok("Status fetched")
}

fn simulated_reqwest_get(url: &str) -> String {
    format!("Response from {}", url)
}
