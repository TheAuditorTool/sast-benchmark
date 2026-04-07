pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let value = parse_json_value(&body);
    let valid = validate_schema(&value);
    if valid {
        super::shared::BenchmarkResponse::ok("Valid")
    } else {
        super::shared::BenchmarkResponse::bad_request("Schema violation")
    }
}

fn parse_json_value(_json: &str) -> String { "value".to_string() }
fn validate_schema(_value: &str) -> bool { true }
