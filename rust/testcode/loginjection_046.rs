//! CWE-117: Constant-folded condition always routes to sanitized log path.

// vuln-code-snippet start testcodeLoginjection046
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    if 5 * 5 == 25 {
        let safe = username.replace('\n', "\\n").replace('\r', "\\r");
        log_info(&format!("user={}", safe)); // vuln-code-snippet target-line testcodeLoginjection046
    } else {
        log_info(&format!("user={}", username));
    }
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection046
