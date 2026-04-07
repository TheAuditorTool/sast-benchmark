pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let mut creds = std::collections::HashMap::new();
    creds.insert("hardcoded", "fake-key-placeholder".to_string());
    creds.insert("env_key", std::env::var("API_KEY").unwrap_or_default());
    let key = creds.get("env_key").unwrap();
    let result = format!("GET {} Authorization: Bearer {}", endpoint, key);
    super::shared::BenchmarkResponse::ok(&result)
}
