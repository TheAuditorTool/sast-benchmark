//! CWE-330: Sequential counter used as auth token.

use std::sync::atomic::{AtomicU64, Ordering};

// vuln-code-snippet start testcodeWeakrand003
static COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = COUNTER.fetch_add(1, Ordering::Relaxed); // vuln-code-snippet target-line testcodeWeakrand003

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}
// vuln-code-snippet end testcodeWeakrand003
