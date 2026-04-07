//! CWE-532: User password written to application log file.

// vuln-code-snippet start testcodeInfodisclosure026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let password = req.param("password");
    log_debug(&format!("Login: user={} pass={}", username, password)); // vuln-code-snippet target-line testcodeInfodisclosure026
    super::shared::BenchmarkResponse::ok("Login recorded")
}

fn log_debug(msg: &str) { eprintln!("[DEBUG] {}", msg); }
// vuln-code-snippet end testcodeInfodisclosure026
