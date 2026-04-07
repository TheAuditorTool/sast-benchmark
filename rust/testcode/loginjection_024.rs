//! CWE-117: Search query written to audit log without stripping injected log entries.

// vuln-code-snippet start testcodeLoginjection024
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let query = req.param("q");
    audit_log(&format!("SEARCH query={}", query)); // vuln-code-snippet target-line testcodeLoginjection024
    super::shared::BenchmarkResponse::ok("Searching")
}

fn audit_log(msg: &str) {
    eprintln!("[AUDIT] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection024
