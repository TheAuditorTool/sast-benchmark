pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let api_key = std::env::var("API_KEY").unwrap_or_else(|_| String::new());
    let result = format!("GET {} Authorization: Bearer {}", endpoint, api_key);
    super::shared::BenchmarkResponse::ok(&result)
}
