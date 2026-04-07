pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    if url.len() >= 200 {
        return super::shared::BenchmarkResponse::bad_request("URL too long");
    }

    let _resp = simulated_reqwest_get(&url);

    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn simulated_reqwest_get(url: &str) -> String {
    format!("Response from {}", url)
}
