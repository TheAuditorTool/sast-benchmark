//! CWE-330: Token derived from the current Unix timestamp, which is low-entropy and predictable.

use std::time::{SystemTime, UNIX_EPOCH};

// vuln-code-snippet start testcodeWeakrand014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = SystemTime::now() // vuln-code-snippet target-line testcodeWeakrand014
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    super::shared::BenchmarkResponse::ok(&format!("Token: {:016x}", token))
}
// vuln-code-snippet end testcodeWeakrand014
