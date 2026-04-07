pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let service = req.param("service");
    let url = format!("http://{}/health", service);

    let _resp = simulated_reqwest_get(&url);

    super::shared::BenchmarkResponse::ok(&format!("Health checked: {}", service))
}

fn simulated_reqwest_get(url: &str) -> String {
    format!("Response from {}", url)
}
