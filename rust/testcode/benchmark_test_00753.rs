pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let resource = req.param("resource");

    if !resource.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return super::shared::BenchmarkResponse::bad_request("Invalid resource identifier");
    }

    let url = format!("https://api.example.com/data/{}", resource);

    let _resp = simulated_reqwest_get(&url);

    super::shared::BenchmarkResponse::ok(&format!("Fetched resource: {}", resource))
}

fn simulated_reqwest_get(url: &str) -> String {
    format!("Response from {}", url)
}
