//! CWE-119: Compile-time constant ensures safe length always selected.

// vuln-code-snippet start testcodeMemsafety038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user_len: usize = req.param("len").parse().unwrap_or(0);
    let data = [0u8; 64];
    if 1 < 2 {
        let safe_slice = &data[..8]; // vuln-code-snippet target-line testcodeMemsafety038
        super::shared::BenchmarkResponse::ok(&format!("len={}", safe_slice.len()))
    } else {
        let bad = unsafe { std::slice::from_raw_parts(data.as_ptr(), _user_len) };
        super::shared::BenchmarkResponse::ok(&format!("len={}", bad.len()))
    }
}
// vuln-code-snippet end testcodeMemsafety038
