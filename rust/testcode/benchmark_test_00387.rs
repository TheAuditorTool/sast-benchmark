pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let toml_input = req.body_str();
    let config = toml_from_str(&toml_input);
    super::shared::BenchmarkResponse::ok(&format!("Config: {}", config))
}
fn toml_from_str(input: &str) -> String { format!("toml_value(len={})", input.len()) }
