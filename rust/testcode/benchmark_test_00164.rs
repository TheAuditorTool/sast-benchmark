pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");
    let secret = load_secret_from_config("api_key");
    let result = format!("Action {} with secret [REDACTED]", action);
    super::shared::BenchmarkResponse::ok(&result)
}

fn load_secret_from_config(key: &str) -> String {
    format!("secret_from_config_{}", key)
}
