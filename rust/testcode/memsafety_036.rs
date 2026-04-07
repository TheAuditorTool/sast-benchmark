//! CWE-119: Slice helper discards user length and returns slice of hardcoded safe size.

// vuln-code-snippet start testcodeMemsafety036
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let len = req.param("len");
    let data = [0u8; 64];
    let slice = safe_slice(&data, &len); // vuln-code-snippet target-line testcodeMemsafety036
    super::shared::BenchmarkResponse::ok(&format!("len={}", slice.len()))
}

fn safe_slice<'a>(buf: &'a [u8], _user_len: &str) -> &'a [u8] {
    &buf[..8]
}
// vuln-code-snippet end testcodeMemsafety036
