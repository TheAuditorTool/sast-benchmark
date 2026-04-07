pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let feed = req.param("feed");

    let _resp = simulated_reqwest_get(&feed);

    super::shared::BenchmarkResponse::ok(&format!("Fetched feed: {}", feed))
}

fn simulated_reqwest_get(url: &str) -> String {
    format!("Response from {}", url)
}
