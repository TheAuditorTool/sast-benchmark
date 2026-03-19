//! Weak Random True Positive — CWE-330
//! Time-seeded RNG for session ID. SystemTime seed is predictable
//! — attacker can guess the seed and reproduce all session IDs.

use std::time::SystemTime;

// vuln-code-snippet start testcodeWeakrand002Vulnerable
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    // VULNERABLE: SystemTime seed is predictable — session IDs reproducible
    let seed = SystemTime::now() // vuln-code-snippet vuln-line testcodeWeakrand002Vulnerable
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let session_id = format!("sess_{:016x}", seed.wrapping_mul(6364136223846793005));
    super::shared::BenchmarkResponse::ok(&format!("Session: {}", session_id))
}
// vuln-code-snippet end testcodeWeakrand002Vulnerable
