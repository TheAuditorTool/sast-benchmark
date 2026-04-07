pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");
    let config = load_config();
    let result = format!("Action {} with config loaded", action);
    super::shared::BenchmarkResponse::ok(&result)
}

fn load_config() -> String {
    "config_loaded".to_string()
}
