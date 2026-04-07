//! CWE-532: Full SQL query string including user data written to debug log.

// vuln-code-snippet start testcodeInfodisclosure030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("username");
    let query = format!("SELECT * FROM users WHERE name = '{}'", user);
    log_debug(&format!("Executing: {}", query)); // vuln-code-snippet target-line testcodeInfodisclosure030
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_debug(msg: &str) { eprintln!("[DEBUG] {}", msg); }
// vuln-code-snippet end testcodeInfodisclosure030
