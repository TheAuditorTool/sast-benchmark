pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let yaml_input = req.body_str();

    let parsed: serde_yaml::Value = serde_yaml::from_str(&yaml_input)
        .unwrap_or(serde_yaml::Value::Null);

    super::shared::BenchmarkResponse::ok(&format!("Parsed YAML: {:?}", parsed))
}
