//! CWE-117: Tracing subscriber configured with JSON formatter that auto-escapes field values.

// vuln-code-snippet start testcodeLoginjection012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("data");

    // Simulates: tracing_subscriber::fmt().json().init() -- JSON formatter auto-escapes
    json_trace("request", &user_input); // vuln-code-snippet target-line testcodeLoginjection012

    super::shared::BenchmarkResponse::ok("Logged")
}

fn json_trace(event: &str, field: &str) {
    // Simulates: tracing with JSON subscriber -- values are JSON-escaped automatically
    let escaped = field.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n").replace('\r', "\\r");
    eprintln!(r#"{{"event":"{}","field":"{}"}}"#, event, escaped);
}
// vuln-code-snippet end testcodeLoginjection012
