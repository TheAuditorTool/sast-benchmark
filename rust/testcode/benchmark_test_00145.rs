pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let host = req.param("host");
    let url = format!("http://{}/api/data", host);

    let _resp = simulated_reqwest_get(&url);

    super::shared::BenchmarkResponse::ok(&format!("Fetched from host: {}", host))
}

fn simulated_reqwest_get(url: &str) -> String {
    format!("Response from {}", url)
}
