//! CWE-117: Structured log entry uses typed numeric field preventing string injection.

// vuln-code-snippet start testcodeLoginjection044
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id: u64 = req.param("user_id").parse().unwrap_or(0);
    let event_code: u32 = req.param("event_code").parse().unwrap_or(0);
    log_structured(user_id, event_code); // vuln-code-snippet target-line testcodeLoginjection044
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_structured(user_id: u64, event_code: u32) {
    eprintln!("[STRUCT] user_id={} event_code={}", user_id, event_code);
}
// vuln-code-snippet end testcodeLoginjection044
