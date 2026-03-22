//! CWE-330: Time-seeded RNG for session ID.

use std::time::SystemTime;

// vuln-code-snippet start testcodeWeakrand002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let seed = SystemTime::now() // vuln-code-snippet target-line testcodeWeakrand002
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let session_id = format!("sess_{:016x}", seed.wrapping_mul(6364136223846793005));
    super::shared::BenchmarkResponse::ok(&format!("Session: {}", session_id))
}
// vuln-code-snippet end testcodeWeakrand002
