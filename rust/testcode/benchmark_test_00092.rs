pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let json_input = req.body_str();

    let data: serde_json::Value = serde_json::from_str(&json_input)
        .unwrap_or(serde_json::Value::Null);

    let role = data.get("role").and_then(|v| v.as_str()).unwrap_or("user");
    super::shared::BenchmarkResponse::ok(&format!("Role: {}", role))
}
