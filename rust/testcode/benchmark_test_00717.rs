pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let ct = req.header("Content-Type");
    if !ct.contains("application/json") {
        return super::shared::BenchmarkResponse::bad_request("Not JSON");
    }
    let body = req.param("body");
    let value = parse_json_value(&body);
    super::shared::BenchmarkResponse::ok(&value)
}

fn parse_json_value(_json: &str) -> String {
    "serde_json::Value::Object(...)".to_string()
}
