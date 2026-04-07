//! CWE-117: Username sanitized by stripping CR and LF before being written to log.

// vuln-code-snippet start testcodeLoginjection036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let safe = username.replace('\n', "\\n").replace('\r', "\\r");
    log_info(&format!("Login: user={}", safe)); // vuln-code-snippet target-line testcodeLoginjection036
    super::shared::BenchmarkResponse::ok("Logged")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection036
