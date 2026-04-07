//! CWE-330: Sequential atomic counter used as auth token — predictable sequence.

use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(1);

// vuln-code-snippet start testcodeWeakrand022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = COUNTER.fetch_add(1, Ordering::SeqCst); // vuln-code-snippet target-line testcodeWeakrand022

    super::shared::BenchmarkResponse::ok(&format!("Token: {:016x}", token))
}
// vuln-code-snippet end testcodeWeakrand022
