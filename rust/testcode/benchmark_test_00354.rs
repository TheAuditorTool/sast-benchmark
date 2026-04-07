pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("data");

    json_trace("request", &user_input);

    super::shared::BenchmarkResponse::ok("Logged")
}

fn json_trace(event: &str, field: &str) {
    let escaped = field.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n").replace('\r', "\\r");
    eprintln!(r#"{{"event":"{}","field":"{}"}}"#, event, escaped);
}
