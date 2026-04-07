pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let full_url = format!("http://{}/api/data", endpoint);
    super::shared::BenchmarkResponse::ok(&format!("Request to: {}", full_url))
}
