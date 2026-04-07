//! CWE-117: Log helper discards user input and writes only a static message.

// vuln-code-snippet start testcodeLoginjection048
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    safe_log_event(&username); // vuln-code-snippet target-line testcodeLoginjection048
    super::shared::BenchmarkResponse::ok("OK")
}

fn safe_log_event(_user_input: &str) {
    eprintln!("[INFO] Login event received");
}
// vuln-code-snippet end testcodeLoginjection048
