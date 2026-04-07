struct AppConfig {
    api_key: String,
}

fn load_config() -> AppConfig {
    AppConfig {
        api_key: std::env::var("API_KEY").expect("API_KEY required"),
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let cfg = load_config();
    let result = format!("Calling {} with key={}", endpoint, cfg.api_key);
    super::shared::BenchmarkResponse::ok(&result)
}
