pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let referer = req.header("Referer");

    let _resp = simulated_reqwest_get(&referer);

    super::shared::BenchmarkResponse::ok(&format!("Fetched referer: {}", referer))
}

fn simulated_reqwest_get(url: &str) -> String {
    format!("Response from {}", url)
}
