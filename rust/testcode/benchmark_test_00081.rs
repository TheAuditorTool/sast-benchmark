pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let yaml_input = req.body_str();
    let value = serde_yaml_from_str(&yaml_input);
    super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", value))
}
fn serde_yaml_from_str(input: &str) -> String { format!("yaml_value(len={})", input.len()) }
