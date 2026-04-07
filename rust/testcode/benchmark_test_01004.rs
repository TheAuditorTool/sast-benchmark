struct AppConfig {
    api_key: &'static str,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let endpoint = req.param("endpoint");
    let cfg = AppConfig { api_key: "hardcoded-prod-key-xyz789abc" };
    let result = format!("Calling {} with key={}", endpoint, cfg.api_key);
    super::shared::BenchmarkResponse::ok(&result)
}
