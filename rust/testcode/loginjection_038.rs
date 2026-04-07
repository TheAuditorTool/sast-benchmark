//! CWE-117: Only the parsed numeric user ID is logged, not the raw user-supplied string.

// vuln-code-snippet start testcodeLoginjection038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id: u64 = req.param("user_id").parse().unwrap_or(0);
    log_info(&format!("Login: user_id={}", user_id)); // vuln-code-snippet target-line testcodeLoginjection038
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection038
