//! CWE-117: User input passed to log-formatting helper that writes it without sanitization.

// vuln-code-snippet start testcodeLoginjection029
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let event = req.param("event");
    log_user_event("INFO", &event); // vuln-code-snippet target-line testcodeLoginjection029
    super::shared::BenchmarkResponse::ok("Recorded")
}

fn log_user_event(level: &str, event: &str) {
    eprintln!("[{}] user_event={}", level, event);
}
// vuln-code-snippet end testcodeLoginjection029
