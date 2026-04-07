pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let safe_string = validate_utf8(&body);
    super::shared::BenchmarkResponse::ok(&format!("received_len={}", safe_string.len()))
}

fn validate_utf8(s: &str) -> &str {
    s
}
