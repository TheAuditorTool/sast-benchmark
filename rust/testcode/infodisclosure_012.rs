//! CWE-209: Stack backtrace included in HTTP error response body.

// vuln-code-snippet start testcodeInfodisclosure012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");
    let backtrace = capture_backtrace(); // vuln-code-snippet target-line testcodeInfodisclosure012
    super::shared::BenchmarkResponse::error(&format!("Error: {}", backtrace))
}
fn capture_backtrace() -> String {
    // Simulates: std::backtrace::Backtrace::capture()
    "at app::handler (src/main.rs:42)\nat actix_web::server (actix-web/src/server.rs:108)".to_string()
}
// vuln-code-snippet end testcodeInfodisclosure012
