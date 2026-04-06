//! CWE-117: HTTP header value written to file-based log appender without sanitization.

// vuln-code-snippet start testcodeLoginjection006
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let referer = req.header("referer");

    // Simulates: log4rs file appender
    file_log(&format!("[AUDIT] referer={}", referer)); // vuln-code-snippet target-line testcodeLoginjection006

    super::shared::BenchmarkResponse::ok("Logged")
}

fn file_log(msg: &str) {
    // Simulates: writing to log4rs file appender
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().append(true).open("/var/log/app.log") {
        let _ = writeln!(f, "{}", msg);
    }
}
// vuln-code-snippet end testcodeLoginjection006
