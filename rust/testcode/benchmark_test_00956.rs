pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let value_str = req.param("value");

    let value: i64 = match value_str.parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Value must be a valid integer"),
    };

    super::shared::BenchmarkResponse::ok(&format!("Received value: {}", value))
}
