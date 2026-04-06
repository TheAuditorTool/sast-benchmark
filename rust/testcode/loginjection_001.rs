//! CWE-117: User-supplied login identifier written directly into application log.

// vuln-code-snippet start testcodeLoginjection001
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("username");

    // Simulates: log::info!()
    log_info(&format!("Login attempt: user={}", user_input)); // vuln-code-snippet target-line testcodeLoginjection001

    super::shared::BenchmarkResponse::ok(&format!("Login recorded for: {}", user_input))
}

fn log_info(msg: &str) {
    // Simulates: log::info!("{}", msg)
    eprintln!("[INFO] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection001
