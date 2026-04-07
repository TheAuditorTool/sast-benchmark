pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("data");

    let log_json = serde_json_to_string("event", &user_input);
    eprintln!("{}", log_json);

    super::shared::BenchmarkResponse::ok("Logged")
}

fn serde_json_to_string(key: &str, value: &str) -> String {
    let escaped = value.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n");
    format!(r#"{{"{}":"{}"}}"#, key, escaped)
}
