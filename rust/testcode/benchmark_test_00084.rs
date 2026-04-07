pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user_url = req.param("url");

    let target = if 7 * 6 > 40 {
        "https://api.example.com/data".to_string()
    } else {
        req.param("url")
    };

    let _resp = simulated_reqwest_get(&target);

    super::shared::BenchmarkResponse::ok("Fetched")
}

fn simulated_reqwest_get(url: &str) -> String {
    format!("Response from {}", url)
}
