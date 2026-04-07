pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let json_input = req.body_str();
    let value = permissive_deser(&json_input);
    super::shared::BenchmarkResponse::ok(&format!("Data: {}", value))
}
fn permissive_deser(input: &str) -> String {
    format!("any_value(len={})", input.len())
}
