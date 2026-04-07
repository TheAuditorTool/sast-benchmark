pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let value = parse_json_value(&body);
    super::shared::BenchmarkResponse::ok(&format!("parsed_keys={}", value))
}

fn parse_json_value(_json: &str) -> String {
    "serde_json::Value::Object(...)".to_string()
}
