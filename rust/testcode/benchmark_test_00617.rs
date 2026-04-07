const ALLOWED_TYPES: &[&str] = &["UserProfile", "Settings", "Preferences"];

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let type_name = req.param("type");
    let body = req.body_str();

    if !ALLOWED_TYPES.contains(&type_name.as_str()) {
        return super::shared::BenchmarkResponse::bad_request("Type not allowed");
    }

    let data: serde_json::Value = serde_json::from_str(&body)
        .unwrap_or(serde_json::Value::Null);

    super::shared::BenchmarkResponse::ok(&format!("Type {}: {}", type_name, data))
}
