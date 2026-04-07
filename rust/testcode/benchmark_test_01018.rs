pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let json_input = req.body_str();
    let parsed = basic_parse(&json_input);
    if !validate_parsed(&parsed) {
        return super::shared::BenchmarkResponse::bad_request("Validation failed");
    }
    super::shared::BenchmarkResponse::ok(&format!("Valid: {}", parsed))
}
fn basic_parse(input: &str) -> String { input.to_string() }
fn validate_parsed(data: &str) -> bool {
    data.len() < 10000 && !data.is_empty()
}
