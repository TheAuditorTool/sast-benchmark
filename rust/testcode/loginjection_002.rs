//! CWE-117: Request path included as tracing span field value.

// vuln-code-snippet start testcodeLoginjection002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let request_path = req.param("path");

    // Simulates: tracing::info!(user = %request_path, "Request received")
    tracing_info("request", &request_path); // vuln-code-snippet target-line testcodeLoginjection002

    super::shared::BenchmarkResponse::ok("Logged")
}

fn tracing_info(event: &str, field: &str) {
    // Simulates: tracing::info!(field = %value)
    eprintln!("[TRACE] {}: {}", event, field);
}
// vuln-code-snippet end testcodeLoginjection002
