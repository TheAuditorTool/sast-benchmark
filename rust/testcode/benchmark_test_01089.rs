fn get_api_key() -> String {
    std::env::var("API_KEY").expect("API_KEY required")
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let key = get_api_key();
    let result = format!("GET {} Authorization: Bearer {}", endpoint, key);
    super::shared::BenchmarkResponse::ok(&result)
}
