//! Weak Random True Positive — CWE-330
//! PID + timestamp as request ID. Both are observable by attacker,
//! making the combined value predictable and forgeable.

use std::time::SystemTime;

// vuln-code-snippet start testcodeWeakrand004Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    let pid = std::process::id();
    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    // VULNERABLE: PID + timestamp are both observable — predictable IDs
    let request_id = format!("req_{}_{}", pid, ts); // vuln-code-snippet vuln-line testcodeWeakrand004Vulnerable

    super::shared::BenchmarkResponse::ok(&format!("RequestID: {}", request_id))
}
// vuln-code-snippet end testcodeWeakrand004Vulnerable
