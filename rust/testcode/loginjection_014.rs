//! CWE-117: Log entry serialized via serde_json::to_string which auto-escapes values.

// vuln-code-snippet start testcodeLoginjection014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("data");

    let log_json = serde_json_to_string("event", &user_input); // vuln-code-snippet target-line testcodeLoginjection014
    eprintln!("{}", log_json);

    super::shared::BenchmarkResponse::ok("Logged")
}

fn serde_json_to_string(key: &str, value: &str) -> String {
    // Simulates: serde_json::to_string(&log_entry) -- auto-escapes special chars
    let escaped = value.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n");
    format!(r#"{{"{}":"{}"}}"#, key, escaped)
}
// vuln-code-snippet end testcodeLoginjection014
