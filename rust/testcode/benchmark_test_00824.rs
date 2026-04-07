pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("data");

    json_fields_trace("input", &user_input);

    super::shared::BenchmarkResponse::ok("Logged")
}

fn json_fields_trace(key: &str, value: &str) {
    let escaped = value.replace('\\', "\\\\").replace('"', "\\\"")
        .replace('\n', "\\n").replace('\r', "\\r").replace('\t', "\\t");
    eprintln!(r#"{{"{}":"{}"}}"#, key, escaped);
}
