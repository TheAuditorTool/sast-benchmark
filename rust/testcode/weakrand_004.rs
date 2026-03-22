//! CWE-330: PID + timestamp as request ID.

use std::time::SystemTime;

// vuln-code-snippet start testcodeWeakrand004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    let pid = std::process::id();
    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    let request_id = format!("req_{}_{}", pid, ts); // vuln-code-snippet target-line testcodeWeakrand004

    super::shared::BenchmarkResponse::ok(&format!("RequestID: {}", request_id))
}
// vuln-code-snippet end testcodeWeakrand004
