pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let yaml = req.param("yaml");
    let result = yaml_deserialize(&yaml);
    super::shared::BenchmarkResponse::ok(&result)
}

fn yaml_deserialize(_yaml: &str) -> String {
    "yaml_object".to_string()
}
