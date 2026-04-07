//! CWE-117: User-supplied username logged without sanitizing newline characters.

// vuln-code-snippet start testcodeLoginjection021
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    log_info(&format!("Login attempt: user={}", username)); // vuln-code-snippet target-line testcodeLoginjection021
    super::shared::BenchmarkResponse::ok("Logged")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection021
