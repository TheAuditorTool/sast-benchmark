//! CWE-117: Log fields collected into Vec including raw user input, then joined and written.

// vuln-code-snippet start testcodeLoginjection028
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("user");
    let mut fields = vec!["event=login".to_string()];
    fields.push(format!("user={}", user));
    fields.push("result=ok".to_string());
    let log_line = fields.join(" ");
    log_info(&log_line); // vuln-code-snippet target-line testcodeLoginjection028
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection028
