//! CWE-117: Log message contains only static strings; no user-supplied data included.

// vuln-code-snippet start testcodeLoginjection039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _username = req.param("username");
    log_info("Login attempt received"); // vuln-code-snippet target-line testcodeLoginjection039
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection039
