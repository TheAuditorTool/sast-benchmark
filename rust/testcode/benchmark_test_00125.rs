pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");

    let _resp = simulated_reqwest_get(&endpoint);

    super::shared::BenchmarkResponse::ok(&format!("Called endpoint: {}", endpoint))
}

fn simulated_reqwest_get(url: &str) -> String {
    format!("Response from {}", url)
}
