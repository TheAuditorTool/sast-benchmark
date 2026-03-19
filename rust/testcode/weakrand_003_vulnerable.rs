//! Weak Random True Positive — CWE-330
//! Sequential counter used as auth token. Completely predictable —
//! attacker can enumerate all valid tokens trivially.

use std::sync::atomic::{AtomicU64, Ordering};

// vuln-code-snippet start testcodeWeakrand003Vulnerable
static COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    // VULNERABLE: Sequential counter — trivially predictable tokens
    let token = COUNTER.fetch_add(1, Ordering::Relaxed); // vuln-code-snippet vuln-line testcodeWeakrand003Vulnerable

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}
// vuln-code-snippet end testcodeWeakrand003Vulnerable
