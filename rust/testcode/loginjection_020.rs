//! CWE-117: Tracing subscriber with JsonFields formatter that escapes all field values.

// vuln-code-snippet start testcodeLoginjection020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("data");

    // Simulates: tracing_subscriber::fmt().json().with_current_span(false).init()
    json_fields_trace("input", &user_input); // vuln-code-snippet target-line testcodeLoginjection020

    super::shared::BenchmarkResponse::ok("Logged")
}

fn json_fields_trace(key: &str, value: &str) {
    // Simulates: tracing_subscriber::fmt::format::JsonFields -- auto-escapes all values
    let escaped = value.replace('\\', "\\\\").replace('"', "\\\"")
        .replace('\n', "\\n").replace('\r', "\\r").replace('\t', "\\t");
    eprintln!(r#"{{"{}":"{}"}}"#, key, escaped);
}
// vuln-code-snippet end testcodeLoginjection020
