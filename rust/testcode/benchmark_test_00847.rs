pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_url = req.param("url");

    let _resp = fetch_status(&user_url);

    super::shared::BenchmarkResponse::ok("Status retrieved")
}

fn fetch_status(_user_url: &str) -> String {
    simulated_reqwest_get("https://api.example.com/status")
}

fn simulated_reqwest_get(url: &str) -> String {
    format!("Response from {}", url)
}
