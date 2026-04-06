//! CWE-117: Raw User-Agent header value logged in structured field.

// vuln-code-snippet start testcodeLoginjection004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let ua = req.header("user-agent");

    // Simulates: slog::info!(logger, "request"; "user_agent" => &ua)
    slog_info("request", "user_agent", &ua); // vuln-code-snippet target-line testcodeLoginjection004

    super::shared::BenchmarkResponse::ok("Logged")
}

fn slog_info(event: &str, key: &str, val: &str) {
    // Simulates: slog structured logger
    eprintln!("[SLOG] {} {}={}", event, key, val);
}
// vuln-code-snippet end testcodeLoginjection004
