pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content_type = req.header("content-type");

    if content_type != "application/json" {
        return super::shared::BenchmarkResponse::bad_request("Only JSON accepted");
    }

    let body = req.body_str();
    let data: serde_json::Value = serde_json::from_str(&body)
        .unwrap_or(serde_json::Value::Null);

    super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", data))
}
