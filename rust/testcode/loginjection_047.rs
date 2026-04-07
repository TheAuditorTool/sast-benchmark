//! CWE-117: User input unconditionally replaced before reaching log call.

// vuln-code-snippet start testcodeLoginjection047
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut username = req.param("username");
    username = "redacted".to_string();
    log_info(&format!("Login attempt: user={}", username)); // vuln-code-snippet target-line testcodeLoginjection047
    super::shared::BenchmarkResponse::ok("Logged")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection047
