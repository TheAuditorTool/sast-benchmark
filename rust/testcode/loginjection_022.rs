//! CWE-117: Client IP address logged directly without sanitizing embedded newlines.

// vuln-code-snippet start testcodeLoginjection022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let ip = req.header("X-Forwarded-For");
    log_warn(&format!("Request from: {}", ip)); // vuln-code-snippet target-line testcodeLoginjection022
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_warn(msg: &str) {
    eprintln!("[WARN] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection022
