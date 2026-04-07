//! CWE-117: User input stored in HashMap but safe constant value read and logged instead.

use std::collections::HashMap;

// vuln-code-snippet start testcodeLoginjection050
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut entries = HashMap::new();
    entries.insert("user_input", req.param("username"));
    entries.insert("safe_msg", "login_event".to_string());
    let msg = entries.get("safe_msg").unwrap();
    log_info(&format!("event={}", msg)); // vuln-code-snippet target-line testcodeLoginjection050
    super::shared::BenchmarkResponse::ok("OK")
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection050
