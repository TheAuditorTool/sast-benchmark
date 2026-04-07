//! CWE-532: Full JWT access token written to debug log.

// vuln-code-snippet start testcodeInfodisclosure027
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");
    log_debug(&format!("Auth token: {}", token)); // vuln-code-snippet target-line testcodeInfodisclosure027
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_debug(msg: &str) { eprintln!("[DEBUG] {}", msg); }
// vuln-code-snippet end testcodeInfodisclosure027
