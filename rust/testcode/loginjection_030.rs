//! CWE-117: Requested file path written to access log allowing fake log-entry injection.

// vuln-code-snippet start testcodeLoginjection030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");
    let ip = req.header("X-Real-IP");
    let log_entry = format!("{} GET {}", ip, path);
    access_log(&log_entry); // vuln-code-snippet target-line testcodeLoginjection030
    super::shared::BenchmarkResponse::ok("Served")
}

fn access_log(entry: &str) {
    eprintln!("[ACCESS] {}", entry);
}
// vuln-code-snippet end testcodeLoginjection030
