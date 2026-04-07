pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");

    let _resp = simulated_client_get_with_redirects(&url);

    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn simulated_client_get_with_redirects(url: &str) -> String {
    format!("Response from {} (redirects followed)", url)
}
