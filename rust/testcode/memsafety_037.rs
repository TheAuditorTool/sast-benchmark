//! CWE-119: User length stored in HashMap; safe constant length read and used for slice.

use std::collections::HashMap;

// vuln-code-snippet start testcodeMemsafety037
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut lens = HashMap::new();
    lens.insert("user_len", req.param("len").parse::<usize>().unwrap_or(0));
    lens.insert("safe_len", 8usize);
    let len = *lens.get("safe_len").unwrap();
    let data = [0u8; 64];
    let slice = unsafe { std::slice::from_raw_parts(data.as_ptr(), len) }; // vuln-code-snippet target-line testcodeMemsafety037
    super::shared::BenchmarkResponse::ok(&format!("len={}", slice.len()))
}
// vuln-code-snippet end testcodeMemsafety037
