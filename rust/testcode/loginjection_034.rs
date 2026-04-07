//! CWE-117: HTTP Referer header value logged without sanitization.

// vuln-code-snippet start testcodeLoginjection034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let referer = req.header("Referer");
    log_access(&format!("referer={}", referer)); // vuln-code-snippet target-line testcodeLoginjection034
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_access(msg: &str) {
    eprintln!("[ACCESS] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection034
