//! CWE-119: Vec::set_len called with user-controlled length; uninitialized memory exposed.

// vuln-code-snippet start testcodeMemsafety017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_len: usize = req.param("len").parse().unwrap_or(0);
    let mut v: Vec<u8> = Vec::with_capacity(64);
    unsafe { v.set_len(user_len) }; // vuln-code-snippet target-line testcodeMemsafety017
    super::shared::BenchmarkResponse::ok(&format!("len={}", v.len()))
}
// vuln-code-snippet end testcodeMemsafety017
