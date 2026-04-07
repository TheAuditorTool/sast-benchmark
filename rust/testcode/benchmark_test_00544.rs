pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _body = req.param("body");
    let config_json = load_server_config();
    let config = deserialize_typed_struct(config_json);
    match config {
        Ok(c) => super::shared::BenchmarkResponse::ok(&c),
        Err(e) => super::shared::BenchmarkResponse::error(&e),
    }
}

fn load_server_config() -> &'static str { r#"{"version":"1.0"}"# }
fn deserialize_typed_struct(_json: &str) -> Result<String, String> {
    Ok("Config { version: 1.0 }".to_string())
}
