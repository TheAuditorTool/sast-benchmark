//! CWE-117: Dedicated sanitizer strips all ASCII control characters before log write.

// vuln-code-snippet start testcodeLoginjection042
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let safe = sanitize_for_log(&username);
    log_info(&format!("user={}", safe)); // vuln-code-snippet target-line testcodeLoginjection042
    super::shared::BenchmarkResponse::ok("OK")
}

fn sanitize_for_log(s: &str) -> String {
    s.chars().filter(|c| !c.is_ascii_control()).collect()
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection042
