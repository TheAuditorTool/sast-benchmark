//! CWE-117: Raw request body written to audit trail file.

// vuln-code-snippet start testcodeLoginjection008
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.body_str();
    let user = req.param("user");

    // Simulates: writeln!(audit_file, ...)
    audit_log(&format!("Action by {}: {}", user, body)); // vuln-code-snippet target-line testcodeLoginjection008

    super::shared::BenchmarkResponse::ok("Audited")
}

fn audit_log(msg: &str) {
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().append(true).open("/var/log/audit.log") {
        let _ = writeln!(f, "{}", msg);
    }
}
// vuln-code-snippet end testcodeLoginjection008
