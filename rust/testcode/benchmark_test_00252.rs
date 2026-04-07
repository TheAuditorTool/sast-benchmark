fn access_system_config() -> String {
    "db_password=secret;api_key=abc123".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _key = req.param("key");
    let result = access_system_config();
    super::shared::BenchmarkResponse::ok(&result)
}
