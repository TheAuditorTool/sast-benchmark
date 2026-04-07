fn read_config_file(key_name: &str) -> String {
    std::env::var("CONFIG_PATH")
        .map(|p| format!("loaded '{}' from {}", key_name, p))
        .unwrap_or_default()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let service = req.param("service");
    let config_value = read_config_file(&service);
    let result = format!("Config for {}: {}", service, config_value);
    super::shared::BenchmarkResponse::ok(&result)
}
