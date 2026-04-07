//! CWE-117: Event type validated as alphanumeric before logging; arbitrary strings rejected.

// vuln-code-snippet start testcodeLoginjection040
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let event_type = req.param("event");
    if !event_type.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return super::shared::BenchmarkResponse::bad_request("Invalid event type");
    }
    log_info(&format!("event={}", event_type)); // vuln-code-snippet target-line testcodeLoginjection040
    super::shared::BenchmarkResponse::ok("Logged")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection040
