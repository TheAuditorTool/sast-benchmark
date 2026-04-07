//! CWE-117: Username truncated and control characters removed before inclusion in log.

// vuln-code-snippet start testcodeLoginjection045
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.param("username");
    let safe: String = raw.chars()
        .filter(|c| !c.is_ascii_control())
        .take(32)
        .collect();
    log_info(&format!("login user={}", safe)); // vuln-code-snippet target-line testcodeLoginjection045
    super::shared::BenchmarkResponse::ok("Logged")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection045
