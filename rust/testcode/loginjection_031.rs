//! CWE-117: Log sanitization strips LF but leaves CR, enabling CRLF log injection.

// vuln-code-snippet start testcodeLoginjection031
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("username");
    let sanitized = user.replace('\n', "\\n");
    log_info(&format!("Login: user={}", sanitized)); // vuln-code-snippet target-line testcodeLoginjection031
    super::shared::BenchmarkResponse::ok("Logged")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection031
