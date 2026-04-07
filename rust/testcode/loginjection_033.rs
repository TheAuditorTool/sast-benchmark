//! CWE-117: HTML encoding applied to username but does not neutralize log newline injection.

// vuln-code-snippet start testcodeLoginjection033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let encoded = username.replace('<', "&lt;").replace('>', "&gt;");
    log_info(&format!("Login: user={}", encoded)); // vuln-code-snippet target-line testcodeLoginjection033
    super::shared::BenchmarkResponse::ok("Logged")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection033
