fn get_real_key(_placeholder: &str) -> String {
    std::env::var("API_KEY").unwrap_or_default()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let key = get_real_key("ignored-hardcoded-value");
    let result = format!("GET {} Authorization: Bearer {}", endpoint, key);
    super::shared::BenchmarkResponse::ok(&result)
}
